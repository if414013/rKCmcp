#!/usr/bin/env python3
"""
Keycloak Documentation Indexer

Scrapes Keycloak documentation and indexes it into Milvus for semantic search.
"""

import os
import sys
import json
import hashlib
import re
from dataclasses import dataclass, field, asdict
from typing import List, Optional
from urllib.parse import urljoin

import requests
from bs4 import BeautifulSoup

KEYCLOAK_DOCS_URLS = [
    "https://www.keycloak.org/docs/latest/server_admin/index.html",
    "https://www.keycloak.org/docs/latest/server_development/index.html",
    "https://www.keycloak.org/docs/latest/authorization_services/index.html",
    "https://www.keycloak.org/docs/latest/upgrading/index.html",
]

DOC_TYPE_MAP = {
    "server_admin": "Server Administration",
    "server_development": "Server Development",
    "authorization_services": "Authorization Services",
    "upgrading": "Upgrading Guide",
}


@dataclass
class DocumentChunk:
    id: str
    text: str
    source_url: str
    doc_type: str
    section_path: List[str]
    heading: str
    heading_level: int


class MilvusClient:
    def __init__(self, host: str, port: int):
        self.base_url = f"http://{host}:{port}/v2/vectordb"

    def create_collection(self, name: str, dimension: int):
        url = f"{self.base_url}/collections/create"
        payload = {
            "collectionName": name,
            "dimension": dimension,
            "metricType": "COSINE",
            "primaryFieldName": "id",
            "vectorFieldName": "vector",
            "idType": "VarChar",
            "autoId": False,
            "params": {"max_length": 512},
        }
        resp = requests.post(url, json=payload)
        result = resp.json()
        if result.get("code") != 0 and "already exists" not in result.get(
            "message", ""
        ):
            raise Exception(f"Failed to create collection: {result}")
        print(f"Collection '{name}' ready")

    def insert(self, collection: str, data: List[dict]):
        url = f"{self.base_url}/entities/insert"
        payload = {"collectionName": collection, "data": data}
        resp = requests.post(url, json=payload)
        result = resp.json()
        if result.get("code") != 0:
            raise Exception(f"Failed to insert: {result}")
        return result.get("data", {}).get("insertCount", 0)


class EmbeddingService:
    def __init__(self, model: str, dimension: int, api_key: Optional[str] = None):
        self.model = model
        self.dimension = dimension
        self.api_key = api_key

    def embed(self, texts: List[str]) -> List[List[float]]:
        if self.api_key:
            return self._embed_openai(texts)
        return self._embed_local(texts)

    def _embed_openai(self, texts: List[str]) -> List[List[float]]:
        resp = requests.post(
            "https://api.openai.com/v1/embeddings",
            headers={
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
            },
            json={"model": self.model, "input": texts},
        )
        result = resp.json()
        if "error" in result:
            raise Exception(f"OpenAI API error: {result['error']}")
        return [d["embedding"] for d in result["data"]]

    def _embed_local(self, texts: List[str]) -> List[List[float]]:
        embeddings = []
        for text in texts:
            embedding = self._simple_hash_embedding(text)
            embeddings.append(embedding)
        return embeddings

    def _simple_hash_embedding(self, text: str) -> List[float]:
        embedding = [0.0] * self.dimension
        words = text.lower().split()
        for i, word in enumerate(words):
            h = int(hashlib.md5(word.encode()).hexdigest(), 16)
            for j in range(8):
                idx = ((h >> (j * 8)) + i) % self.dimension
                val = ((h >> (j * 4)) & 0xFF) / 255.0 - 0.5
                embedding[idx] += val
        magnitude = sum(x * x for x in embedding) ** 0.5
        if magnitude > 0:
            embedding = [x / magnitude for x in embedding]
        return embedding


def extract_doc_type(url: str) -> str:
    for key in DOC_TYPE_MAP:
        if key in url:
            return key
    return "unknown"


def scrape_documentation(url: str) -> List[DocumentChunk]:
    print(f"Scraping: {url}")
    resp = requests.get(url, timeout=60)
    resp.raise_for_status()

    soup = BeautifulSoup(resp.text, "html.parser")
    doc_type = extract_doc_type(url)
    chunks = []

    for script in soup(["script", "style", "nav"]):
        script.decompose()

    toc = soup.find(id="toc")
    if toc:
        toc.decompose()

    sections = soup.find_all(class_=re.compile(r"^sect\d$"))

    for section in sections:
        heading_tag = section.find(re.compile(r"^h[1-6]$"))
        if not heading_tag:
            continue

        heading = heading_tag.get_text(strip=True)
        heading_level = int(heading_tag.name[1])
        section_id = heading_tag.get("id", "")

        section_path = [heading]
        parent = section.parent
        while parent:
            parent_class = parent.get("class", [])
            if any(c.startswith("sect") for c in parent_class):
                parent_heading = parent.find(re.compile(r"^h[1-6]$"))
                if parent_heading:
                    section_path.insert(0, parent_heading.get_text(strip=True))
            parent = parent.parent

        body = section.find(class_="sectionbody")
        if body:
            text = body.get_text(separator="\n", strip=True)
        else:
            text = section.get_text(separator="\n", strip=True)

        text = re.sub(r"\n{3,}", "\n\n", text)
        text = text[:4000]

        if len(text) < 50:
            continue

        chunk_id = hashlib.sha256(f"{url}#{section_id}".encode()).hexdigest()[:32]

        chunk = DocumentChunk(
            id=chunk_id,
            text=text,
            source_url=f"{url}#{section_id}" if section_id else url,
            doc_type=doc_type,
            section_path=section_path,
            heading=heading,
            heading_level=heading_level,
        )
        chunks.append(chunk)

    print(f"  Extracted {len(chunks)} sections")
    return chunks


def chunk_text(text: str, chunk_size: int = 512, overlap: int = 50) -> List[str]:
    if len(text) <= chunk_size:
        return [text]

    chunks = []
    start = 0
    while start < len(text):
        end = start + chunk_size
        chunk = text[start:end]

        if end < len(text):
            last_period = chunk.rfind(". ")
            last_newline = chunk.rfind("\n")
            break_point = max(last_period, last_newline)
            if break_point > chunk_size // 2:
                chunk = chunk[: break_point + 1]
                end = start + break_point + 1

        chunks.append(chunk.strip())
        start = end - overlap

    return chunks


def main():
    milvus_host = os.environ.get("MILVUS_HOST", "localhost")
    milvus_port = int(os.environ.get("MILVUS_PORT", "19530"))
    collection_name = os.environ.get("MILVUS_COLLECTION_DOCS", "keycloak_docs")
    embedding_model = os.environ.get("EMBEDDING_MODEL", "text-embedding-3-small")
    embedding_dimension = int(os.environ.get("EMBEDDING_DIMENSION", "384"))
    openai_api_key = os.environ.get("OPENAI_API_KEY")

    docs_urls_env = os.environ.get("KEYCLOAK_DOCS_URLS")
    if docs_urls_env:
        docs_urls = [u.strip() for u in docs_urls_env.split(",")]
    else:
        docs_urls = KEYCLOAK_DOCS_URLS

    print("=" * 60)
    print("Keycloak Documentation Indexer")
    print("=" * 60)
    print(f"Milvus: {milvus_host}:{milvus_port}")
    print(f"Collection: {collection_name}")
    print(f"Embedding Model: {embedding_model}")
    print(f"Dimension: {embedding_dimension}")
    print(f"URLs to index: {len(docs_urls)}")
    print("=" * 60)

    milvus = MilvusClient(milvus_host, milvus_port)
    embeddings = EmbeddingService(embedding_model, embedding_dimension, openai_api_key)

    milvus.create_collection(collection_name, embedding_dimension)

    all_chunks = []
    for url in docs_urls:
        try:
            chunks = scrape_documentation(url)
            all_chunks.extend(chunks)
        except Exception as e:
            print(f"  Error scraping {url}: {e}")

    print(f"\nTotal chunks to index: {len(all_chunks)}")

    batch_size = 50
    total_indexed = 0

    for i in range(0, len(all_chunks), batch_size):
        batch = all_chunks[i : i + batch_size]
        texts = [c.text for c in batch]

        print(f"Embedding batch {i // batch_size + 1}...")
        vectors = embeddings.embed(texts)

        data = []
        for chunk, vector in zip(batch, vectors):
            data.append(
                {
                    "id": chunk.id,
                    "vector": vector,
                    "text": chunk.text[:2000],
                    "source_url": chunk.source_url,
                    "doc_type": chunk.doc_type,
                    "section_path": json.dumps(chunk.section_path),
                    "heading": chunk.heading,
                    "heading_level": chunk.heading_level,
                }
            )

        count = milvus.insert(collection_name, data)
        total_indexed += count
        print(f"  Indexed {count} chunks")

    print("=" * 60)
    print(f"Indexing complete! Total indexed: {total_indexed}")
    print("=" * 60)


if __name__ == "__main__":
    main()

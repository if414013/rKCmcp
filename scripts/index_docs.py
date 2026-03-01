#!/usr/bin/env python3
"""
Keycloak Documentation Indexer

Crawls and indexes Keycloak documentation into Milvus for semantic search.
Performs full site crawling to discover all documentation pages.
"""

import os
import sys
import json
import hashlib
import re
import time
from collections import deque
from dataclasses import dataclass
from typing import List, Optional, Set
from urllib.parse import urljoin, urlparse, urldefrag

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


class DocumentationCrawler:
    def __init__(self, seed_urls: List[str], max_pages: int = 500, delay: float = 0.5):
        self.seed_urls = seed_urls
        self.max_pages = max_pages
        self.delay = delay
        self.visited: Set[str] = set()
        self.session = requests.Session()
        self.session.headers.update(
            {"User-Agent": "KeycloakMCP-Indexer/1.0 (Documentation Crawler)"}
        )

    def normalize_url(self, url: str) -> str:
        url, _ = urldefrag(url)
        parsed = urlparse(url)
        path = parsed.path.rstrip("/")
        if not path.endswith(".html") and not path.endswith("/"):
            if "." not in path.split("/")[-1]:
                path = path + "/index.html"
        return f"{parsed.scheme}://{parsed.netloc}{path}"

    def is_valid_doc_url(self, url: str, base_url: str) -> bool:
        parsed = urlparse(url)
        base_parsed = urlparse(base_url)

        if parsed.netloc != base_parsed.netloc:
            return False

        if not parsed.path.startswith("/docs/"):
            return False

        if any(
            ext in parsed.path
            for ext in [
                ".css",
                ".js",
                ".png",
                ".jpg",
                ".gif",
                ".svg",
                ".ico",
                ".pdf",
                ".zip",
            ]
        ):
            return False

        return True

    def extract_links(self, soup: BeautifulSoup, current_url: str) -> List[str]:
        links = []
        for a_tag in soup.find_all("a", href=True):
            href = a_tag["href"]

            if (
                href.startswith("#")
                or href.startswith("mailto:")
                or href.startswith("javascript:")
            ):
                continue

            absolute_url = urljoin(current_url, href)
            normalized = self.normalize_url(absolute_url)

            if self.is_valid_doc_url(normalized, current_url):
                links.append(normalized)

        return links

    def crawl(self) -> List[str]:
        queue = deque()
        discovered_urls = []

        for seed_url in self.seed_urls:
            normalized = self.normalize_url(seed_url)
            if normalized not in self.visited:
                queue.append(normalized)
                self.visited.add(normalized)

        print(f"Starting crawl with {len(queue)} seed URLs...")
        print(f"Max pages: {self.max_pages}")

        while queue and len(discovered_urls) < self.max_pages:
            url = queue.popleft()

            try:
                print(f"  Crawling: {url}")
                resp = self.session.get(url, timeout=30)
                resp.raise_for_status()

                discovered_urls.append(url)

                soup = BeautifulSoup(resp.text, "html.parser")
                new_links = self.extract_links(soup, url)

                for link in new_links:
                    if link not in self.visited:
                        self.visited.add(link)
                        queue.append(link)

                if self.delay > 0:
                    time.sleep(self.delay)

            except Exception as e:
                print(f"    Error crawling {url}: {e}")
                continue

        print(f"Crawl complete. Discovered {len(discovered_urls)} pages.")
        return discovered_urls


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


def scrape_page(url: str, session: requests.Session) -> List[DocumentChunk]:
    try:
        resp = session.get(url, timeout=60)
        resp.raise_for_status()
    except Exception as e:
        print(f"    Error fetching {url}: {e}")
        return []

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

    return chunks


def main():
    milvus_host = os.environ.get("MILVUS_HOST", "localhost")
    milvus_port = int(os.environ.get("MILVUS_PORT", "19530"))
    collection_name = os.environ.get("MILVUS_COLLECTION_DOCS", "keycloak_docs")
    embedding_model = os.environ.get("EMBEDDING_MODEL", "text-embedding-3-small")
    embedding_dimension = int(os.environ.get("EMBEDDING_DIMENSION", "384"))
    openai_api_key = os.environ.get("OPENAI_API_KEY")
    max_pages = int(os.environ.get("MAX_CRAWL_PAGES", "500"))
    crawl_delay = float(os.environ.get("CRAWL_DELAY", "0.3"))

    docs_urls_env = os.environ.get("KEYCLOAK_DOCS_URLS")
    if docs_urls_env:
        docs_urls = [u.strip() for u in docs_urls_env.split(",")]
    else:
        docs_urls = KEYCLOAK_DOCS_URLS

    print("=" * 60)
    print("Keycloak Documentation Indexer (Full Crawler)")
    print("=" * 60)
    print(f"Milvus: {milvus_host}:{milvus_port}")
    print(f"Collection: {collection_name}")
    print(f"Embedding Model: {embedding_model}")
    print(f"Dimension: {embedding_dimension}")
    print(f"Seed URLs: {len(docs_urls)}")
    print(f"Max Pages: {max_pages}")
    print(f"Crawl Delay: {crawl_delay}s")
    print("=" * 60)

    crawler = DocumentationCrawler(docs_urls, max_pages=max_pages, delay=crawl_delay)
    discovered_urls = crawler.crawl()

    if not discovered_urls:
        print("No pages discovered. Exiting.")
        return

    milvus = MilvusClient(milvus_host, milvus_port)
    embeddings = EmbeddingService(embedding_model, embedding_dimension, openai_api_key)

    milvus.create_collection(collection_name, embedding_dimension)

    print("\n" + "=" * 60)
    print("Extracting content from discovered pages...")
    print("=" * 60)

    session = requests.Session()
    session.headers.update(
        {"User-Agent": "KeycloakMCP-Indexer/1.0 (Documentation Crawler)"}
    )

    all_chunks = []
    for i, url in enumerate(discovered_urls):
        print(f"[{i + 1}/{len(discovered_urls)}] Processing: {url}")
        chunks = scrape_page(url, session)
        all_chunks.extend(chunks)
        print(f"    Extracted {len(chunks)} sections")

    unique_chunks = {}
    for chunk in all_chunks:
        if chunk.id not in unique_chunks:
            unique_chunks[chunk.id] = chunk
    all_chunks = list(unique_chunks.values())

    print(f"\nTotal unique chunks to index: {len(all_chunks)}")

    if not all_chunks:
        print("No content extracted. Exiting.")
        return

    batch_size = 50
    total_indexed = 0

    print("\n" + "=" * 60)
    print("Indexing into Milvus...")
    print("=" * 60)

    for i in range(0, len(all_chunks), batch_size):
        batch = all_chunks[i : i + batch_size]
        texts = [c.text for c in batch]

        print(
            f"Embedding batch {i // batch_size + 1}/{(len(all_chunks) + batch_size - 1) // batch_size}..."
        )
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

    print("\n" + "=" * 60)
    print(f"INDEXING COMPLETE!")
    print(f"  Pages crawled: {len(discovered_urls)}")
    print(f"  Chunks indexed: {total_indexed}")
    print("=" * 60)


if __name__ == "__main__":
    main()

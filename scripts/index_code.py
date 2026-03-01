#!/usr/bin/env python3
"""
Keycloak Source Code Indexer

Clones Keycloak repository and indexes Java source code into Milvus for semantic search.
"""

import os
import sys
import json
import hashlib
import subprocess
import re
from dataclasses import dataclass
from typing import List, Optional
from pathlib import Path

import requests

KEYCLOAK_REPO = "https://github.com/keycloak/keycloak.git"
DEFAULT_TAG = "26.0.5"


@dataclass
class CodeChunk:
    id: str
    text: str
    file_path: str
    language: str
    chunk_type: str
    name: Optional[str]
    parent_name: Optional[str]
    start_line: int
    end_line: int
    git_tag: str


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


def clone_repository(repo_url: str, tag: str, target_dir: Path) -> Path:
    if target_dir.exists():
        print(f"Repository already exists at {target_dir}")
        current_tag = subprocess.run(
            ["git", "describe", "--tags", "--exact-match"],
            cwd=target_dir,
            capture_output=True,
            text=True,
        )
        if current_tag.returncode == 0 and current_tag.stdout.strip() == tag:
            print(f"Already at tag {tag}")
            return target_dir
        print(f"Checking out tag {tag}...")
        subprocess.run(["git", "fetch", "--tags"], cwd=target_dir, check=True)
        subprocess.run(["git", "checkout", tag], cwd=target_dir, check=True)
        return target_dir

    print(f"Cloning {repo_url} at tag {tag}...")
    subprocess.run(
        ["git", "clone", "--depth", "1", "--branch", tag, repo_url, str(target_dir)],
        check=True,
    )
    return target_dir


def parse_java_file(file_path: Path, git_tag: str) -> List[CodeChunk]:
    chunks = []

    try:
        content = file_path.read_text(encoding="utf-8", errors="ignore")
    except Exception as e:
        print(f"  Error reading {file_path}: {e}")
        return chunks

    lines = content.split("\n")
    relative_path = str(file_path)

    package_match = re.search(r"package\s+([\w.]+);", content)
    package_name = package_match.group(1) if package_match else ""

    class_pattern = re.compile(
        r"^(\s*)(public\s+)?(abstract\s+)?(final\s+)?(class|interface|enum|record)\s+(\w+)",
        re.MULTILINE,
    )

    method_pattern = re.compile(
        r"^(\s*)(public|protected|private)?\s*(static\s+)?(final\s+)?(synchronized\s+)?"
        r"([\w<>\[\],\s]+)\s+(\w+)\s*\([^)]*\)\s*(throws\s+[\w,\s]+)?\s*\{",
        re.MULTILINE,
    )

    for match in class_pattern.finditer(content):
        class_type = match.group(5)
        class_name = match.group(6)
        start_pos = match.start()
        start_line = content[:start_pos].count("\n") + 1

        brace_count = 0
        end_line = start_line
        in_class = False

        for i, line in enumerate(lines[start_line - 1 :], start=start_line):
            brace_count += line.count("{") - line.count("}")
            if "{" in line and not in_class:
                in_class = True
            if in_class and brace_count == 0:
                end_line = i
                break
            if i - start_line > 500:
                end_line = start_line + 100
                break

        class_text = "\n".join(lines[start_line - 1 : min(end_line, start_line + 100)])

        chunk_id = hashlib.sha256(
            f"{relative_path}:{class_name}:{start_line}".encode()
        ).hexdigest()[:32]

        chunks.append(
            CodeChunk(
                id=chunk_id,
                text=class_text[:4000],
                file_path=relative_path,
                language="java",
                chunk_type=class_type,
                name=class_name,
                parent_name=package_name,
                start_line=start_line,
                end_line=end_line,
                git_tag=git_tag,
            )
        )

    for match in method_pattern.finditer(content):
        return_type = match.group(6).strip() if match.group(6) else ""
        method_name = match.group(7)
        start_pos = match.start()
        start_line = content[:start_pos].count("\n") + 1

        brace_count = 0
        end_line = start_line
        in_method = False

        for i, line in enumerate(lines[start_line - 1 :], start=start_line):
            brace_count += line.count("{") - line.count("}")
            if "{" in line and not in_method:
                in_method = True
            if in_method and brace_count == 0:
                end_line = i
                break
            if i - start_line > 200:
                end_line = start_line + 50
                break

        method_text = "\n".join(lines[start_line - 1 : end_line])

        if len(method_text) < 20:
            continue

        chunk_id = hashlib.sha256(
            f"{relative_path}:{method_name}:{start_line}".encode()
        ).hexdigest()[:32]

        parent_class = None
        for chunk in chunks:
            if chunk.chunk_type in ("class", "interface", "enum", "record"):
                if chunk.start_line < start_line < chunk.end_line:
                    parent_class = chunk.name
                    break

        chunks.append(
            CodeChunk(
                id=chunk_id,
                text=method_text[:4000],
                file_path=relative_path,
                language="java",
                chunk_type="method",
                name=method_name,
                parent_name=parent_class,
                start_line=start_line,
                end_line=end_line,
                git_tag=git_tag,
            )
        )

    return chunks


def index_repository(repo_path: Path, git_tag: str) -> List[CodeChunk]:
    all_chunks = []

    java_files = list(repo_path.rglob("*.java"))
    java_files = [f for f in java_files if "test" not in str(f).lower()]
    java_files = [f for f in java_files if "target" not in str(f)]

    print(f"Found {len(java_files)} Java source files (excluding tests)")

    for i, java_file in enumerate(java_files):
        if i % 100 == 0:
            print(f"Processing file {i + 1}/{len(java_files)}...")

        chunks = parse_java_file(java_file, git_tag)
        all_chunks.extend(chunks)

    return all_chunks


def main():
    milvus_host = os.environ.get("MILVUS_HOST", "localhost")
    milvus_port = int(os.environ.get("MILVUS_PORT", "19530"))
    collection_name = os.environ.get("MILVUS_COLLECTION_CODE", "keycloak_code")
    embedding_model = os.environ.get("EMBEDDING_MODEL", "text-embedding-3-small")
    embedding_dimension = int(os.environ.get("EMBEDDING_DIMENSION", "384"))
    openai_api_key = os.environ.get("OPENAI_API_KEY")
    git_tag = os.environ.get("KEYCLOAK_REPO_TAG", DEFAULT_TAG)
    cache_dir = Path(os.environ.get("CACHE_DIR", "/app/cache"))

    print("=" * 60)
    print("Keycloak Source Code Indexer")
    print("=" * 60)
    print(f"Milvus: {milvus_host}:{milvus_port}")
    print(f"Collection: {collection_name}")
    print(f"Embedding Model: {embedding_model}")
    print(f"Dimension: {embedding_dimension}")
    print(f"Keycloak Tag: {git_tag}")
    print("=" * 60)

    milvus = MilvusClient(milvus_host, milvus_port)
    embeddings = EmbeddingService(embedding_model, embedding_dimension, openai_api_key)

    milvus.create_collection(collection_name, embedding_dimension)

    repo_path = cache_dir / "keycloak"
    clone_repository(KEYCLOAK_REPO, git_tag, repo_path)

    print("\nParsing Java source files...")
    all_chunks = index_repository(repo_path, git_tag)
    print(f"\nTotal chunks to index: {len(all_chunks)}")

    batch_size = 50
    total_indexed = 0

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
                    "file_path": chunk.file_path,
                    "language": chunk.language,
                    "chunk_type": chunk.chunk_type,
                    "name": chunk.name or "",
                    "parent_name": chunk.parent_name or "",
                    "start_line": chunk.start_line,
                    "end_line": chunk.end_line,
                    "git_tag": chunk.git_tag,
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

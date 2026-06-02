import asyncio
from pathlib import Path
from typing import List, Optional
from tempfile import gettempdir
from aiofile import async_open

from dotenv import load_dotenv
load_dotenv()

async def walk_directory_async(
    directory: str | Path,
    extensions: Optional[List[str]] = None,
    recursive: bool = True,
) -> List[Path]:
    base_path = Path(directory)
    if extensions is not None:
        normalized_extensions = set(ext.lower() for ext in extensions)
    else:
        normalized_extensions = None

    # Get all files asynchronously using thread pool (pathlib is synchronous)
    def get_files():
        path_iterator = base_path.rglob("*") if recursive else base_path.glob("*")
        return [p for p in path_iterator if p.is_file()]
    
    all_files = await asyncio.to_thread(get_files)
    
    # Filter by extensions
    matched_files = [
        f for f in all_files 
        if normalized_extensions is None or f.suffix.lower() in normalized_extensions
    ]
    
    # Write to cache concurrently with lock for thread safety
    write_lock = asyncio.Lock()
    async with async_open("./walk_directory_cache.txt", "w") as cache_file:
        async def write_file(file_path: Path):
            async with write_lock:
                await cache_file.write(str(file_path) + "\n")
            return file_path
        
        await asyncio.gather(
            *[write_file(f) for f in matched_files]
        )
    
    return matched_files


async def main():
    directory = r"C:\Users\sunil\Downloads\HPM Docs\03 Design Control Templates"
    extensions = [".docx", ".pdf"]
    matched_files = await walk_directory_async(directory, extensions)
    for file in matched_files:
        print(file)

if __name__ == "__main__":
    asyncio.run(main())



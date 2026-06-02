import asyncio
from pathlib import Path
from filewalk import walk_directory_async

from markitdown import MarkItDown


async def convert_file(
    file_path: Path,
    output_format: str = "markdown",
    ):

    # outdir = file_path.parent / "converted"
    # outdir.mkdir(exist_ok=True)

    md = MarkItDown()
    result = md.convert(
        source=file_path,
        target=file_path.with_suffix(".md"),
        format=output_format
    )

    print(f"Converted {file_path} to {result}")


async def main():
    directory = r"C:\Users\sunil\Downloads\HPM Docs\03 Design Control Templates"
    # file_path = Path(r"C:\Users\sunil\Downloads\HPM Docs\OneDrive_2026-04-13\Cyient Shared Folder\Finished Project DHF\01683_D001899780 PIC iX 4.5.1 Product Safety Risk Management Report Rev C.docx")
    # file_path = Path(r"C:\Users\sunil\Downloads\HPM Docs\03 Design Control templates\A-Q2920-01683-T005_C.pdf")
    # file_path = Path(r"C:\Users\sunil\Downloads\HPM Docs\03 Design Control templates\2001001513_Template.pdf")
    # file_path = Path(r"C:\Users\sunil\Downloads\HPM Docs\03 Design Control templates\2001001513_Template.docx")
    # file_path = Path(r"C:\Users\sunil\Downloads\HPM Docs\OneDrive_2026-04-13\Cyient Shared Folder\Finished Project DHF\01678_867214-90003_D002072202 MSVP (SH1.0 MAC Address Master Verification Plan APC D03) RevA_AI_review.docx")
    file_path = Path(r"C:\space\projects\python\markitdown\data\cts_scanning_mr.pdf")

    await convert_file(file_path)
    # extensions = [".docx", ".pdf"]
    # matched_files = await walk_directory_async(directory, extensions)
    # for file in matched_files:
    #     await convert_file(file)

if __name__ == "__main__":
    asyncio.run(main())
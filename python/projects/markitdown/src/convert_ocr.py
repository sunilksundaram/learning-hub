import os
import asyncio
from pathlib import Path
from filewalk import walk_directory_async

from markitdown import MarkItDown
from openai import AzureOpenAI, OpenAI

from dotenv import load_dotenv
load_dotenv()


async def get_llm_client():
    endpoint = os.getenv("OPENAI_ENDPOINT") or "https://buildassist-resource.cognitiveservices.azure.com/"
    subscription_key = os.getenv("SUBSCRIPTION_KEY")
    api_version = os.getenv("API_VERSION")
    model_name = os.getenv("MODEL_NAME") or "gpt-4o-mini"  # Fallback to DEPLOYMENT if MODEL_NAME is not set
    deployment = os.getenv("DEPLOYMENT")

    client = AzureOpenAI(
        azure_endpoint=endpoint,
        api_key=subscription_key,
        api_version=api_version
    )

    response = client.chat.completions.create(
        model=model_name, # Must match deployment name
        messages=[{"role": "user", "content": "Hello"}]
    )
    print(response.choices[0].message.content)

    return client


async def convert_file(
    file_path: Path,
    output_format: str = "markdown",
    ):

    # outdir = file_path.parent / "converted"
    # outdir.mkdir(exist_ok=True)

    llm_model = os.getenv("MODEL_NAME") or "gpt-4o-mini"

    md = MarkItDown(
        enable_plugins=True,
        llm_client=await get_llm_client(),
        llm_model=llm_model,
    )
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
    file_path = Path(r"C:\Users\sunil\Downloads\HPM Docs\OneDrive_2026-04-13\Cyient Shared Folder\Finished Project DHF\01678_867214-90003_D002072202 MSVP (SH1.0 MAC Address Master Verification Plan APC D03) RevA_AI_review.docx")

    await convert_file(file_path)
    # extensions = [".docx", ".pdf"]
    # matched_files = await walk_directory_async(directory, extensions)
    # for file in matched_files:
    #     await convert_file(file)

if __name__ == "__main__":
    asyncio.run(main())
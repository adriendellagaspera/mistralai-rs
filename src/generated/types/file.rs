/**The File object (not file name) to be uploaded.
To upload a file and specify a custom file name you should format your request as such:
```ignorebash
file=@path/to/your/file.jsonl;filename=custom_name.jsonl
```ignore
Otherwise, you can just keep the original file name:
```ignorebash
file=@path/to/your/file.jsonl
```ignore*/
pub type File = bytes::Bytes;

# ap-scraper

Downloads the latest images from Reddit's [/r/ArtPorn] community and saves them to `./imgs`. These images will eventually be hosted in S3 and be exposed via an API in a separate project.

```
A simple program for scraping https://reddit.com/r/artporn for high-resolution art

Usage: ap-scraper [OPTIONS]

Options:
  -o, --outdir <OUTDIR>  The loation to save the file [default: ./imgs]
  -h, --help             Print help information
  -V, --version          Print version information
```

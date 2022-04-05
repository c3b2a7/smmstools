## SM.MS Uploader

A lightweight [SM.MS](https://sm.ms/) upload tool with no external dependencies, no interface, support for multiple file uploads and works with typora.

### Install

```shell
sudo mkdir -p /usr/local/bin && \
sudo wget "https://github.com/c3b2a7/smms-uploader/releases/latest/download/smms-uploader-$(uname -s)-$(uname -m)" -O /usr/local/bin/smms-uploader && \
sudo chmod +x /usr/local/bin/smms-uploader
```

### Usage

```shell
# visit https://sm.ms/home/apitoken to get apitoken
smms-uploader apitoken image1-path image2-path ...
```

### License

MIT


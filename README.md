## SM.MS Uploader

A lightweight [SM.MS](https://sm.ms/) upload tool, no external dependencies, no interface, supports multiple image uploads and can work with typora.

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


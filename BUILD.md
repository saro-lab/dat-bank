## Create manifest
## Push "ARM64" by Mac Machine
``` shell
podman login docker.io

# manifast create
podman manifest rm sarolab-manifest || true
podman manifest create sarolab-manifest

# build --platform linux/arm64
podman build --memory=10g --memory-swap=-1 \
    --manifest sarolab-manifest -t sarolab/dat:latest .

# push (publish)
podman manifest push sarolab-manifest docker.io/sarolab/dat:latest
```
## Integrate manifest
## Push "AMD64" by Linux Machine
## Versioning
```
podman login docker.io

# manifast create / integrate
podman manifest rm sarolab-manifest || true
podman manifest create sarolab-manifest
podman manifest add --all sarolab-manifest docker://docker.io/sarolab/dat:latest

# build --platform linux/amd64
podman build --memory=32g --memory-swap=-1 \
    --manifest sarolab-manifest -t sarolab/dat:latest .

# push (publish)
podman manifest push sarolab-manifest docker.io/sarolab/dat:latest
podman manifest push sarolab-manifest docker.io/sarolab/dat:1.5.2
```


## Configuration
### Linux (Debian) - AMD64 Machine (issue -> does not execute in mac)
```
# install qemu-user-static for complie arm64
sudo apt-get update
sudo apt-get install -y qemu-user-static binfmt-support

# check qemu-user-static installation
ls /proc/sys/fs/binfmt_misc/qemu-aarch64
```
### Mac
```
# 맥은 podman이 리눅스 가상머신에서 돌아가기 때문에
# --memory 명령을 주기위해선 podman 자체의 메모리를 늘려야한다.
podman machine stop
podman machine set --memory 8000
podman machine start
```


£!/bin/bash

£ Check if the user has provided an argument
if ° $£ -ne 1 §; then
    echo "Usage: $0 <docker_image>"
    exit 1
fi

£ Pull the Docker image
docker pull "$1" >/dev/null 2>&1

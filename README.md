# Blink (Led Hello World on a Pi using Rust)

If you have your ssh key setup on the pi with the default pi user just run.
This will build the binary, copy it using ssh and the executing it. The output will
automatically piped over ssh and closing the ssh will automatically kill the process on the pi.

Ensure you have docker and docker-compose available.

`sh deploy.sh`

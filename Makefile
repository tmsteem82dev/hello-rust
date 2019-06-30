build-docker-latest:
	docker build -t hello-rust:latest .

run-docker-latest:
	docker run -p 8000:8000 hello-rust:latest
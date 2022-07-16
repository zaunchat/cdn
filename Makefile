dev:
	docker run -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
	docker run -e MINIO_ROOT_USER=s3-storage -e MINIO_ROOT_PASSWORD=password -p 10000:9000 -d minio/minio server /data
	docker run minio/mc mb minio/attachments minio/avatars minio/icons minio/backgrounds
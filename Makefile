local:
	source ./backend/.env_local  && \
	

local-down:
	docker compose down
compose-start:
	docker compose up -d
compose-stop:
	docker compose down 

build:
	cd backend && \
	docker build -t test:latest .
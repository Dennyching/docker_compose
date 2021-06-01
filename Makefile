
build:
	cd backend && \
	docker build -t test:latest .
migrate:
	source ./backend/.env_local && \
	sqlx db create && \
	sqlx migrate --source backend/migrations run

local:migrate
	

local-down:
	docker compose down
compose-start:
	docker compose up -d
compose-stop:
	docker compose down 

API_TOKEN=05AzkXHiQqzJRG55P/vzAhRCBMIhCyQTw0L5xasK
CURL=curl -H "Authorization: Bearer ${API_TOKEN}"
get1:
	$(CURL) localhost:8000/tasks/1 --silent | jq

getother:
	$(CURL) localhost:8000/tasks/32 -v


list:
	$(CURL) localhost:8000/tasks --silent | jq

post:
	$(CURL) -X POST http://localhost:8000/tasks \
		-H "Content-Type: application/json" \
		-d '{"title": "Write report"}'

update1-fail:
	$(CURL) -X POST http://localhost:8000/tasks/1 \
		-H "Content-Type: application/json" \
		-d '{"label": "report"}' -v

update1:
	$(CURL) -X POST http://localhost:8000/tasks/1 \
		-H "Content-Type: application/json" \
		-d '{"label": "Later"}' -v

auth-fail:
	curl -X POST http://localhost:8000/tasks/1 \
		-H "Content-Type: application/json" \
		-d '{"label": "Later"}' -v

auth-fail2:
	curl -H "Authorization: Bearer fasdfsdjfdskfjdslk" \
		-X POST http://localhost:8000/tasks/1 \
		-H "Content-Type: application/json" \
		-d '{"label": "Later"}' -v

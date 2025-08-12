get42:
	curl localhost:8000/tasks/42 --silent | jq

getother:
	curl localhost:8000/tasks/32 -v


list:
	curl localhost:8000/tasks --silent | jq

post:
	curl -X POST http://localhost:8000/tasks \
		-H "Content-Type: application/json" \
		-d '{"tasktitle": "Write report"}'

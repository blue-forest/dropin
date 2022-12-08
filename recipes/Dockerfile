FROM python:alpine

WORKDIR /docs

COPY requirements.txt ./

RUN pip install --no-cache-dir -r requirements.txt

CMD [ "mkdocs", "serve" ]

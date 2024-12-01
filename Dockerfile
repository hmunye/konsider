FROM postgres:alpine

COPY ./docker/postgresql.conf /etc/postgresql/postgresql.conf

COPY ./certs /etc/ssl/certs

RUN chown postgres:postgres /etc/ssl/certs/server.key

ENTRYPOINT ["docker-entrypoint.sh"]

CMD ["postgres", "-c", "config_file=/etc/postgresql/postgresql.conf"]

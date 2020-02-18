FROM mysql:5.7.12 as production-pseudo

COPY etc/conf.d/mysql.cnf /etc/mysql/conf.d/mysql.cnf

# --

FROM production-pseudo as development

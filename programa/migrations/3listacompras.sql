CREATE TABLE lista_compras (
    produto    INTEGER         REFERENCES produtos (codigo) ON DELETE RESTRICT
                                                            ON UPDATE CASCADE,
    quantidade NUMERIC (16, 3),
    unidade    TEXT,
    codigo     INTEGER         PRIMARY KEY AUTOINCREMENT
)

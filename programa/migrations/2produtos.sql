
CREATE TABLE produtos (
    nome   TEXT,
    codigo INTEGER PRIMARY KEY AUTOINCREMENT,
    grupo  INTEGER REFERENCES grupos (codigo) ON DELETE RESTRICT
                                              ON UPDATE CASCADE
)
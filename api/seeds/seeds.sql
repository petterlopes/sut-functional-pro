-- seeds.sql: valores iniciais para tabelas principais do SUT

-- =========================
-- Localidades
-- =========================
insert into tel_localidade (txnomelocalidade) values
  ('Matriz'),
  ('Filial Norte'),
  ('Filial Sul');

-- =========================
-- Departamentos
-- =========================
insert into tel_departamento (departamento) values
  ('TI'),
  ('RH'),
  ('Financeiro');

-- =========================
-- Relacionamento Depto/RH × Depto/SUT
-- =========================
insert into tel_rel_dept_rh (depto_rh_codigo, incddepartamento) values
  (1001, 1),
  (1002, 2);

-- =========================
-- Referência de Origem do Contato
-- =========================
insert into tel_ref_origem_contato (descricao, formato) values
  ('Email Corporativo', 'email'),
  ('Telefone Fixo', '(99)9999-9999');

-- =========================
-- Origem do Contato
-- =========================
insert into tel_origem_contato (tipo, origem) values
  ('Interno', 'RH'),
  ('Externo', 'Fornecedor');

-- =========================
-- Tipo de Contato
-- =========================
insert into tel_tipo_contato (tipo, descricao) values
  ('Email', 'E-mail institucional'),
  ('Telefone', 'Telefone comercial');

-- =========================
-- Usuários (IdP)
-- =========================
insert into idp_usuario (idp_user_id, colaborador_username, nome_exibicao) values
  ('uuid-1', 'joao.silva', 'João Silva'),
  ('uuid-2', 'maria.souza', 'Maria Souza');

-- =========================
-- Grupos de Permissão
-- =========================
insert into tel_grupo_permissao (grupo) values
  ('Admin'),
  ('Consulta'),
  ('RH');

-- =========================
-- Grupo Membro
-- =========================
insert into tel_grupo_membro (incdgrupo, idp_user_id, incddepartamento, incdlocalidade) values
  (1, 'uuid-1', 1, 1),
  (2, 'uuid-2', 2, 2);

-- =========================
-- Contatos
-- =========================
insert into tel_contato (nome, origem, matricula, incddepartamento, incdlocalidade, status, preferencia) values
  ('João Silva', 'Email Corporativo', 12345, 1, 1, 'At', 'Email Corporativo'),
  ('Maria Souza', 'Telefone Fixo', 54321, 2, 2, 'At', 'Telefone Fixo');

-- =========================
-- Contato Meio
-- =========================
insert into tel_contato_meio (incdcontato, incdtipocontato, valor) values
  (1, 1, 'joao.silva@empresa.com'),
  (1, 2, '(11)1234-5678'),
  (2, 1, 'maria.souza@empresa.com'),
  (2, 2, '(21)8765-4321');

-- =========================
-- Responsável
-- =========================
insert into tel_responsavel (incdcontato, idp_user_id, incddepartamento, incdlocalidade) values
  (1, 'uuid-1', 1, 1),
  (2, 'uuid-2', 2, 2);

-- =========================
-- Sites de Busca
-- =========================
insert into tel_site_busca (site, url) values
  ('Google', 'https://www.google.com'),
  ('LinkedIn', 'https://www.linkedin.com');

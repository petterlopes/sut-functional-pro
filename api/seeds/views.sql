-- views.sql: views para Tela de Navegação e Pesquisa de Permissão

-- View para Tela de Navegação
create or replace view vw_nav_contato as
select
  c.incdcontato,
  c.nome,
  c.origem,
  c.matricula,
  d.departamento,
  l.txnomelocalidade as localidade,
  c.status,
  c.preferencia,
  array_agg(cm.valor) as meios_contato
from tel_contato c
join tel_departamento d on c.incddepartamento = d.incddepartamento
join tel_localidade l on c.incdlocalidade = l.incdlocalidade
left join tel_contato_meio cm on c.incdcontato = cm.incdcontato
group by c.incdcontato, c.nome, c.origem, c.matricula, d.departamento, l.txnomelocalidade, c.status, c.preferencia;

-- View para Pesquisa de Permissão
create or replace view vw_pesquisa_permissao as
select
  gm.idp_user_id,
  u.colaborador_username,
  u.nome_exibicao,
  gm.incdgrupo,
  gp.grupo,
  gm.incddepartamento,
  d.departamento,
  gm.incdlocalidade,
  l.txnomelocalidade
from tel_grupo_membro gm
join idp_usuario u on gm.idp_user_id = u.idp_user_id
join tel_grupo_permissao gp on gm.incdgrupo = gp.incdgrupo
left join tel_departamento d on gm.incddepartamento = d.incddepartamento
left join tel_localidade l on gm.incdlocalidade = l.incdlocalidade;

README - aplicar schema, seeds e views (Postgres)

Passos rápidos (assumindo psql disponível):

1. Criar banco (exemplo):

   psql -U postgres -c "create database sut_db;"

2. Aplicar schema:

   psql -U postgres -d sut_db -f api/seeds/schema.sql

3. Aplicar seeds:

   psql -U postgres -d sut_db -f api/seeds/seeds.sql

4. Aplicar views:

   psql -U postgres -d sut_db -f api/seeds/views.sql

Notas:
- Ajuste usuário/host/porta conforme seu ambiente.
- As views assumem que as tabelas e seeds foram criadas com as constraints corretas.
- Se usar Docker Compose com Postgres, você pode copiar os arquivos para o container e executá-los via psql dentro do container.

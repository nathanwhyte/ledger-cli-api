-- Your SQL goes here

create table ledger_transactions (
  id            serial  primary key,
  date_created  varchar not null,
  memo          varchar not null
);

create table ledger_entries (
  id                 serial  primary key,
  transaction_id     serial  not null,
  bucket             varchar not null,
  currency           varchar ,
  credit             boolean not null default 'f',
  amount             float   not null,
  comment            varchar not null,

  foreign key (transaction_id) references ledger_transactions (id)
);

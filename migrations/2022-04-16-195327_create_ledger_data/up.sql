-- Your SQL goes here

create table ledger_transaction (
  id            serial  primary key,
  date_created  date    not null,
  memo          varchar not null
);

create table ledger_entry (
  id                 serial  primary key,
  transaction_id     serial  not null,
  bucket             varchar not null,
  currency           varchar ,
  credit             boolean not null default 'f',
  amount             float   not null,
  comment            varchar not null,

  foreign key (transaction_id) references ledger_transaction (id)
);

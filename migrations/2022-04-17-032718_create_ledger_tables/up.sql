-- Your SQL goes here

create table ledger_transactions (
  id            serial  primary key,
  date_created  text not null,
  memo          text not null
);

create table ledger_entries (
  id                 serial  primary key,
  transaction_id     serial  not null,
  bucket             text not null,
  currency           text ,
  credit             boolean not null default 'f',
  amount             float   not null,
  comment            text ,

  foreign key (transaction_id) references ledger_transactions (id)
);

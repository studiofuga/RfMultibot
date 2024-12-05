alter table posts
    add resend integer DEFAULT 0;

create index resend
    on posts (resend);

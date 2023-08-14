--
-- PostgreSQL database dump
--

-- Dumped from database version 14.7 (Ubuntu 14.7-1.pgdg20.04+1)
-- Dumped by pg_dump version 14.7 (Ubuntu 14.7-1.pgdg20.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_table_access_method = heap;

--
-- Name: brand_blocklists; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.brand_blocklists (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    account_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    marketplace_id smallint NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: brand_blocklists_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.brand_blocklists_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: brand_blocklists_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.brand_blocklists_id_seq OWNED BY public.brand_blocklists.id;


--
-- Name: failed_jobs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.failed_jobs (
    id bigint NOT NULL,
    uuid character varying(255) NOT NULL,
    connection text NOT NULL,
    queue text NOT NULL,
    payload text NOT NULL,
    exception text NOT NULL,
    failed_at timestamp(0) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


--
-- Name: failed_jobs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.failed_jobs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: failed_jobs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.failed_jobs_id_seq OWNED BY public.failed_jobs.id;


--
-- Name: filters; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.filters (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text NOT NULL,
    filters jsonb NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: filters_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.filters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: filters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.filters_id_seq OWNED BY public.filters.id;


--
-- Name: google_o_auths; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.google_o_auths (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    access_token character varying(255) NOT NULL,
    expires_in integer NOT NULL,
    scope character varying(255) NOT NULL,
    token_type character varying(255) NOT NULL,
    id_token text NOT NULL,
    folder_id character varying(255) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    refresh_token character varying(255) DEFAULT ''::character varying NOT NULL
);


--
-- Name: google_o_auths_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.google_o_auths_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: google_o_auths_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.google_o_auths_id_seq OWNED BY public.google_o_auths.id;


--
-- Name: histories; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.histories (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    user_id bigint NOT NULL,
    marketplace_id smallint NOT NULL,
    item_type_id smallint NOT NULL,
    item_id bigint NOT NULL,
    event_type_id smallint NOT NULL,
    event jsonb NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: histories_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.histories_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: histories_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.histories_id_seq OWNED BY public.histories.id;


--
-- Name: layouts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.layouts (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    description text NOT NULL,
    "order" jsonb NOT NULL,
    widths jsonb NOT NULL,
    colors jsonb NOT NULL,
    visibility jsonb NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    view character varying(255) DEFAULT 'products'::character varying NOT NULL
);


--
-- Name: layouts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.layouts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: layouts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.layouts_id_seq OWNED BY public.layouts.id;


--
-- Name: listing_matches; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.listing_matches (
    id bigint NOT NULL,
    marketplace_id smallint NOT NULL,
    identifier character varying(255) NOT NULL,
    asin character varying(255) NOT NULL,
    is_match boolean NOT NULL,
    account_id bigint NOT NULL,
    user_id bigint NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: listing_matches_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.listing_matches_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: listing_matches_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.listing_matches_id_seq OWNED BY public.listing_matches.id;


--
-- Name: migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.migrations (
    id integer NOT NULL,
    migration character varying(255) NOT NULL,
    batch integer NOT NULL
);


--
-- Name: migrations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.migrations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: migrations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.migrations_id_seq OWNED BY public.migrations.id;


--
-- Name: multipack_corrections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.multipack_corrections (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    account_id bigint NOT NULL,
    asin character varying(255) NOT NULL,
    quantity integer NOT NULL,
    marketplace_id smallint NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: multipack_corrections_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.multipack_corrections_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: multipack_corrections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.multipack_corrections_id_seq OWNED BY public.multipack_corrections.id;


--
-- Name: notes; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.notes (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    account_id bigint NOT NULL,
    note text NOT NULL,
    asin character varying(255) NOT NULL,
    marketplace_id smallint NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: notes_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.notes_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: notes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.notes_id_seq OWNED BY public.notes.id;


--
-- Name: password_resets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.password_resets (
    email character varying(255) NOT NULL,
    token character varying(255) NOT NULL,
    created_at timestamp(0) without time zone
);


--
-- Name: permissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.permissions (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    team smallint NOT NULL,
    scan smallint NOT NULL,
    listing_match smallint NOT NULL,
    multipack_correction smallint NOT NULL,
    note smallint NOT NULL,
    brand_blocklist smallint NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: permissions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.permissions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: permissions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.permissions_id_seq OWNED BY public.permissions.id;


--
-- Name: personal_access_tokens; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.personal_access_tokens (
    id bigint NOT NULL,
    tokenable_type character varying(255) NOT NULL,
    tokenable_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    token character varying(64) NOT NULL,
    abilities text,
    last_used_at timestamp(0) without time zone,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: personal_access_tokens_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.personal_access_tokens_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: personal_access_tokens_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.personal_access_tokens_id_seq OWNED BY public.personal_access_tokens.id;


--
-- Name: product_list_items; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.product_list_items (
    id bigint NOT NULL,
    list_id bigint NOT NULL,
    product_id bigint NOT NULL
);


--
-- Name: product_list_items_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.product_list_items_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: product_list_items_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.product_list_items_id_seq OWNED BY public.product_list_items.id;


--
-- Name: product_lists; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.product_lists (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    scan_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: product_lists_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.product_lists_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: product_lists_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.product_lists_id_seq OWNED BY public.product_lists.id;


--
-- Name: scans; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.scans (
    id bigint NOT NULL,
    account_id bigint NOT NULL,
    marketplace_id bigint NOT NULL,
    status smallint NOT NULL,
    source_type_id smallint NOT NULL,
    source_id bigint NOT NULL,
    name character varying(255) NOT NULL,
    products integer NOT NULL,
    errors integer NOT NULL,
    speed integer NOT NULL,
    supplier_file character varying(255) NOT NULL,
    results character varying(255) NOT NULL,
    options jsonb NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    filename character varying(255) DEFAULT ''::character varying NOT NULL,
    deleted_at timestamp(0) without time zone,
    user_id bigint DEFAULT '0'::bigint NOT NULL,
    lines integer DEFAULT 0 NOT NULL
);


--
-- Name: scans_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.scans_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: scans_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.scans_id_seq OWNED BY public.scans.id;


--
-- Name: selling_partner_key_marketplaces; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.selling_partner_key_marketplaces (
    id bigint NOT NULL,
    selling_partner_key_id bigint NOT NULL,
    marketplace_id bigint NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    deleted_at timestamp(0) without time zone
);


--
-- Name: selling_partner_key_marketplaces_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.selling_partner_key_marketplaces_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: selling_partner_key_marketplaces_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.selling_partner_key_marketplaces_id_seq OWNED BY public.selling_partner_key_marketplaces.id;


--
-- Name: selling_partner_keys; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.selling_partner_keys (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    account_id bigint NOT NULL,
    seller_id character varying(255) NOT NULL,
    access_token character varying(512) NOT NULL,
    refresh_token character varying(512) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    region character varying(255) DEFAULT ''::character varying NOT NULL,
    is_valid boolean DEFAULT true NOT NULL
);


--
-- Name: selling_partner_keys_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.selling_partner_keys_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: selling_partner_keys_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.selling_partner_keys_id_seq OWNED BY public.selling_partner_keys.id;


--
-- Name: subscriptions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.subscriptions (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    stripe_id character varying(255) NOT NULL,
    status character varying(255) NOT NULL,
    plan character varying(255) NOT NULL,
    amount integer NOT NULL,
    "interval" character varying(255) NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: subscriptions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.subscriptions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: subscriptions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.subscriptions_id_seq OWNED BY public.subscriptions.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    email_verified_at timestamp(0) without time zone,
    password character varying(255) NOT NULL,
    remember_token character varying(100),
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    account_id bigint DEFAULT '0'::bigint NOT NULL,
    deleted_at timestamp(0) without time zone,
    stripe_customer_id character varying(255),
    stripe_subscription_id character varying(255),
    stripe_subscription_status character varying(255),
    subscription_plan character varying(255)
);


--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: webhook_calls; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.webhook_calls (
    id bigint NOT NULL,
    name character varying(255) NOT NULL,
    url character varying(255) NOT NULL,
    headers json,
    payload json,
    exception text,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone
);


--
-- Name: webhook_calls_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.webhook_calls_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: webhook_calls_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.webhook_calls_id_seq OWNED BY public.webhook_calls.id;


--
-- Name: brand_blocklists id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.brand_blocklists ALTER COLUMN id SET DEFAULT nextval('public.brand_blocklists_id_seq'::regclass);


--
-- Name: failed_jobs id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.failed_jobs ALTER COLUMN id SET DEFAULT nextval('public.failed_jobs_id_seq'::regclass);


--
-- Name: filters id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.filters ALTER COLUMN id SET DEFAULT nextval('public.filters_id_seq'::regclass);


--
-- Name: google_o_auths id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.google_o_auths ALTER COLUMN id SET DEFAULT nextval('public.google_o_auths_id_seq'::regclass);


--
-- Name: histories id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.histories ALTER COLUMN id SET DEFAULT nextval('public.histories_id_seq'::regclass);


--
-- Name: layouts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.layouts ALTER COLUMN id SET DEFAULT nextval('public.layouts_id_seq'::regclass);


--
-- Name: listing_matches id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.listing_matches ALTER COLUMN id SET DEFAULT nextval('public.listing_matches_id_seq'::regclass);


--
-- Name: migrations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.migrations ALTER COLUMN id SET DEFAULT nextval('public.migrations_id_seq'::regclass);


--
-- Name: multipack_corrections id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.multipack_corrections ALTER COLUMN id SET DEFAULT nextval('public.multipack_corrections_id_seq'::regclass);


--
-- Name: notes id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.notes ALTER COLUMN id SET DEFAULT nextval('public.notes_id_seq'::regclass);


--
-- Name: permissions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.permissions ALTER COLUMN id SET DEFAULT nextval('public.permissions_id_seq'::regclass);


--
-- Name: personal_access_tokens id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.personal_access_tokens ALTER COLUMN id SET DEFAULT nextval('public.personal_access_tokens_id_seq'::regclass);


--
-- Name: product_list_items id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.product_list_items ALTER COLUMN id SET DEFAULT nextval('public.product_list_items_id_seq'::regclass);


--
-- Name: product_lists id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.product_lists ALTER COLUMN id SET DEFAULT nextval('public.product_lists_id_seq'::regclass);


--
-- Name: scans id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scans ALTER COLUMN id SET DEFAULT nextval('public.scans_id_seq'::regclass);


--
-- Name: selling_partner_key_marketplaces id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.selling_partner_key_marketplaces ALTER COLUMN id SET DEFAULT nextval('public.selling_partner_key_marketplaces_id_seq'::regclass);


--
-- Name: selling_partner_keys id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.selling_partner_keys ALTER COLUMN id SET DEFAULT nextval('public.selling_partner_keys_id_seq'::regclass);


--
-- Name: subscriptions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.subscriptions ALTER COLUMN id SET DEFAULT nextval('public.subscriptions_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: webhook_calls id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhook_calls ALTER COLUMN id SET DEFAULT nextval('public.webhook_calls_id_seq'::regclass);


--
-- Name: brand_blocklists brand_blocklists_account_id_name_marketplace_id_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.brand_blocklists
    ADD CONSTRAINT brand_blocklists_account_id_name_marketplace_id_unique UNIQUE (account_id, name, marketplace_id);


--
-- Name: brand_blocklists brand_blocklists_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.brand_blocklists
    ADD CONSTRAINT brand_blocklists_pkey PRIMARY KEY (id);


--
-- Name: failed_jobs failed_jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.failed_jobs
    ADD CONSTRAINT failed_jobs_pkey PRIMARY KEY (id);


--
-- Name: failed_jobs failed_jobs_uuid_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.failed_jobs
    ADD CONSTRAINT failed_jobs_uuid_unique UNIQUE (uuid);


--
-- Name: filters filters_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.filters
    ADD CONSTRAINT filters_pkey PRIMARY KEY (id);


--
-- Name: google_o_auths google_o_auths_account_id_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.google_o_auths
    ADD CONSTRAINT google_o_auths_account_id_unique UNIQUE (account_id);


--
-- Name: google_o_auths google_o_auths_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.google_o_auths
    ADD CONSTRAINT google_o_auths_pkey PRIMARY KEY (id);


--
-- Name: histories histories_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.histories
    ADD CONSTRAINT histories_pkey PRIMARY KEY (id);


--
-- Name: layouts layouts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.layouts
    ADD CONSTRAINT layouts_pkey PRIMARY KEY (id);


--
-- Name: listing_matches listing_matches_marketplace_id_identifier_asin_account_id_uniqu; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.listing_matches
    ADD CONSTRAINT listing_matches_marketplace_id_identifier_asin_account_id_uniqu UNIQUE (marketplace_id, identifier, asin, account_id);


--
-- Name: listing_matches listing_matches_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.listing_matches
    ADD CONSTRAINT listing_matches_pkey PRIMARY KEY (id);


--
-- Name: migrations migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.migrations
    ADD CONSTRAINT migrations_pkey PRIMARY KEY (id);


--
-- Name: multipack_corrections multipack_corrections_account_id_asin_marketplace_id_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.multipack_corrections
    ADD CONSTRAINT multipack_corrections_account_id_asin_marketplace_id_unique UNIQUE (account_id, asin, marketplace_id);


--
-- Name: multipack_corrections multipack_corrections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.multipack_corrections
    ADD CONSTRAINT multipack_corrections_pkey PRIMARY KEY (id);


--
-- Name: notes notes_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.notes
    ADD CONSTRAINT notes_pkey PRIMARY KEY (id);


--
-- Name: permissions permissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.permissions
    ADD CONSTRAINT permissions_pkey PRIMARY KEY (id);


--
-- Name: personal_access_tokens personal_access_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.personal_access_tokens
    ADD CONSTRAINT personal_access_tokens_pkey PRIMARY KEY (id);


--
-- Name: personal_access_tokens personal_access_tokens_token_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.personal_access_tokens
    ADD CONSTRAINT personal_access_tokens_token_unique UNIQUE (token);


--
-- Name: product_list_items product_list_items_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.product_list_items
    ADD CONSTRAINT product_list_items_pkey PRIMARY KEY (id);


--
-- Name: product_list_items product_list_items_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.product_list_items
    ADD CONSTRAINT product_list_items_unique UNIQUE (list_id, product_id);


--
-- Name: product_lists product_lists_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.product_lists
    ADD CONSTRAINT product_lists_pkey PRIMARY KEY (id);


--
-- Name: scans scans_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.scans
    ADD CONSTRAINT scans_pkey PRIMARY KEY (id);


--
-- Name: selling_partner_key_marketplaces selling_partner_key_marketplaces_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.selling_partner_key_marketplaces
    ADD CONSTRAINT selling_partner_key_marketplaces_pkey PRIMARY KEY (id);


--
-- Name: selling_partner_key_marketplaces selling_partner_key_marketplaces_selling_partner_key_id_marketp; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.selling_partner_key_marketplaces
    ADD CONSTRAINT selling_partner_key_marketplaces_selling_partner_key_id_marketp UNIQUE (selling_partner_key_id, marketplace_id);


--
-- Name: selling_partner_keys selling_partner_keys_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.selling_partner_keys
    ADD CONSTRAINT selling_partner_keys_pkey PRIMARY KEY (id);


--
-- Name: selling_partner_keys selling_partner_keys_seller_id_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.selling_partner_keys
    ADD CONSTRAINT selling_partner_keys_seller_id_unique UNIQUE (seller_id);


--
-- Name: subscriptions subscriptions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.subscriptions
    ADD CONSTRAINT subscriptions_pkey PRIMARY KEY (id);


--
-- Name: subscriptions subscriptions_user_id_stripe_id_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.subscriptions
    ADD CONSTRAINT subscriptions_user_id_stripe_id_unique UNIQUE (user_id, stripe_id);


--
-- Name: users users_email_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_unique UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: webhook_calls webhook_calls_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.webhook_calls
    ADD CONSTRAINT webhook_calls_pkey PRIMARY KEY (id);


--
-- Name: brand_blocklists_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX brand_blocklists_account_id_index ON public.brand_blocklists USING btree (account_id);


--
-- Name: brand_blocklists_marketplace_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX brand_blocklists_marketplace_id_index ON public.brand_blocklists USING btree (marketplace_id);


--
-- Name: brand_blocklists_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX brand_blocklists_user_id_index ON public.brand_blocklists USING btree (user_id);


--
-- Name: filters_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX filters_account_id_index ON public.filters USING btree (account_id);


--
-- Name: histories_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX histories_account_id_index ON public.histories USING btree (account_id);


--
-- Name: histories_event_type_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX histories_event_type_id_index ON public.histories USING btree (event_type_id);


--
-- Name: histories_item_type_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX histories_item_type_id_index ON public.histories USING btree (item_type_id);


--
-- Name: histories_marketplace_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX histories_marketplace_id_index ON public.histories USING btree (marketplace_id);


--
-- Name: histories_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX histories_user_id_index ON public.histories USING btree (user_id);


--
-- Name: layouts_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX layouts_account_id_index ON public.layouts USING btree (account_id);


--
-- Name: listing_matches_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX listing_matches_account_id_index ON public.listing_matches USING btree (account_id);


--
-- Name: listing_matches_marketplace_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX listing_matches_marketplace_id_index ON public.listing_matches USING btree (marketplace_id);


--
-- Name: multipack_corrections_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX multipack_corrections_account_id_index ON public.multipack_corrections USING btree (account_id);


--
-- Name: multipack_corrections_marketplace_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX multipack_corrections_marketplace_id_index ON public.multipack_corrections USING btree (marketplace_id);


--
-- Name: multipack_corrections_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX multipack_corrections_user_id_index ON public.multipack_corrections USING btree (user_id);


--
-- Name: notes_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX notes_account_id_index ON public.notes USING btree (account_id);


--
-- Name: notes_asin_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX notes_asin_index ON public.notes USING btree (asin);


--
-- Name: notes_marketplace_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX notes_marketplace_id_index ON public.notes USING btree (marketplace_id);


--
-- Name: notes_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX notes_user_id_index ON public.notes USING btree (user_id);


--
-- Name: password_resets_email_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX password_resets_email_index ON public.password_resets USING btree (email);


--
-- Name: permissions_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX permissions_user_id_index ON public.permissions USING btree (user_id);


--
-- Name: personal_access_tokens_tokenable_type_tokenable_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX personal_access_tokens_tokenable_type_tokenable_id_index ON public.personal_access_tokens USING btree (tokenable_type, tokenable_id);


--
-- Name: product_list_items_list_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX product_list_items_list_id_index ON public.product_list_items USING btree (list_id);


--
-- Name: product_list_items_product_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX product_list_items_product_id_index ON public.product_list_items USING btree (product_id);


--
-- Name: product_lists_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX product_lists_account_id_index ON public.product_lists USING btree (account_id);


--
-- Name: product_lists_scan_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX product_lists_scan_id_index ON public.product_lists USING btree (scan_id);


--
-- Name: scans_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX scans_account_id_index ON public.scans USING btree (account_id);


--
-- Name: scans_status_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX scans_status_index ON public.scans USING btree (status);


--
-- Name: selling_partner_keys_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX selling_partner_keys_account_id_index ON public.selling_partner_keys USING btree (account_id);


--
-- Name: selling_partner_keys_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX selling_partner_keys_user_id_index ON public.selling_partner_keys USING btree (user_id);


--
-- Name: subscriptions_stripe_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX subscriptions_stripe_id_index ON public.subscriptions USING btree (stripe_id);


--
-- Name: subscriptions_user_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX subscriptions_user_id_index ON public.subscriptions USING btree (user_id);


--
-- Name: users_account_id_index; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX users_account_id_index ON public.users USING btree (account_id);


--
-- Name: brand_blocklists brand_blocklists_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.brand_blocklists
    ADD CONSTRAINT brand_blocklists_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: listing_matches listing_matches_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.listing_matches
    ADD CONSTRAINT listing_matches_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: multipack_corrections multipack_corrections_user_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.multipack_corrections
    ADD CONSTRAINT multipack_corrections_user_id_foreign FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--


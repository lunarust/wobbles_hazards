--
-- PostgreSQL database dump
--
-- Dumped from database version 16.11
-- Dumped by pg_dump version 16.11

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: category; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.category (
    id smallint NOT NULL,
    title character varying(255),
    link character varying(255),
    description character varying(255),
    layers character varying(255),
    inserted timestamp with time zone DEFAULT now(),
    id_txt character varying
);


ALTER TABLE public.category OWNER TO hazards;

--
-- Name: eonet_calls; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.eonet_calls (
    id integer NOT NULL,
    method character varying,
    date timestamp without time zone NOT NULL,
    closed timestamp without time zone
);


ALTER TABLE public.eonet_calls OWNER TO hazards;

--
-- Name: eonet_calls_id_seq; Type: SEQUENCE; Schema: public; Owner: hazards
--

CREATE SEQUENCE public.eonet_calls_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.eonet_calls_id_seq OWNER TO hazards;

--
-- Name: eonet_calls_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: hazards
--

ALTER SEQUENCE public.eonet_calls_id_seq OWNED BY public.eonet_calls.id;


--
-- Name: event; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.event (
    id character varying(255) NOT NULL,
    title character varying(255) NOT NULL,
    description character varying(255),
    link character varying(255) NOT NULL,
    inserted timestamp with time zone DEFAULT now(),
    closed timestamp without time zone,
    category_id character varying,
    distance real
);


ALTER TABLE public.event OWNER TO hazards;

--
-- Name: event_source; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.event_source (
    source_id character varying(255) NOT NULL,
    event_id character varying(255) NOT NULL
);


ALTER TABLE public.event_source OWNER TO hazards;

--
-- Name: fire; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.fire (
    instrument character varying(255) NOT NULL,
    confidence character varying(255),
    daynight character varying(255),
    satellite character varying(255),
    geometry point,
    bright_ti4 real NOT NULL,
    frp real,
    distance real,
    typ character varying(255),
    "time" timestamp without time zone NOT NULL
);


ALTER TABLE public.fire OWNER TO hazards;

--
-- Name: geometry; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.geometry (
    dt timestamp without time zone,
    type character varying(255),
    coordinates point NOT NULL,
    event_id character varying(255),
    id integer NOT NULL,
    magnitudevalue real,
    magnitudeunit character varying
);


ALTER TABLE public.geometry OWNER TO hazards;

--
-- Name: geometry_id_seq; Type: SEQUENCE; Schema: public; Owner: hazards
--

CREATE SEQUENCE public.geometry_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.geometry_id_seq OWNER TO hazards;

--
-- Name: geometry_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: hazards
--

ALTER SEQUENCE public.geometry_id_seq OWNED BY public.geometry.id;


--
-- Name: quake; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.quake (
    url character varying(255),
    alert character varying(255),
    code character varying(255) NOT NULL,
    magnitude real,
    geometry point,
    depth real,
    "time" timestamp without time zone,
    distance real
);


ALTER TABLE public.quake OWNER TO hazards;

--
-- Name: source; Type: TABLE; Schema: public; Owner: hazards
--

CREATE TABLE IF NOT EXISTS public.source (
    id character varying(255) NOT NULL,
    url character varying(255) NOT NULL
);


ALTER TABLE public.source OWNER TO hazards;

--
-- Name: eonet_calls id; Type: DEFAULT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.eonet_calls ALTER COLUMN id SET DEFAULT nextval('public.eonet_calls_id_seq'::regclass);


--
-- Name: geometry id; Type: DEFAULT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.geometry ALTER COLUMN id SET DEFAULT nextval('public.geometry_id_seq'::regclass);


--
-- Data for Name: category; Type: TABLE DATA; Schema: public; Owner: hazards
--

COPY public.category (id, title, link, description, layers, inserted, id_txt) FROM stdin;
6	Drought	https://eonet.gsfc.nasa.gov/api/v2.1/categories/6	Long lasting absence of precipitation affecting agriculture and livestock, and the overall availability of food and water.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/6	\N	drought
7	Dust and Haze	https://eonet.gsfc.nasa.gov/api/v2.1/categories/7	Related to dust storms, air pollution and other non-volcanic aerosols. Volcano-related plumes shall be included with the originating eruption event.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/7	\N	dustHaze
8	Wildfires	https://eonet.gsfc.nasa.gov/api/v2.1/categories/8	Wildland fires includes all nature of fire, in forest and plains, as well as those that spread to become urban and industrial fire events. Fires may be naturally caused or manmade.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/8	\N	wildfires
9	Floods	https://eonet.gsfc.nasa.gov/api/v2.1/categories/9	Related to aspects of actual flooding--e.g., inundation, water extending beyond river and lake extents.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/9	\N	floods
10	Severe Storms	https://eonet.gsfc.nasa.gov/api/v2.1/categories/10	Related to the atmospheric aspect of storms (hurricanes, cyclones, tornadoes, etc.). Results of storms may be included under floods, landslides, etc.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/10	\N	severeStorms
12	Volcanoes	https://eonet.gsfc.nasa.gov/api/v2.1/categories/12	Related to both the physical effects of an eruption (rock, ash, lava) and the atmospheric (ash and gas plumes).	https://eonet.gsfc.nasa.gov/api/v2.1/layers/12	\N	volcanoes
13	Water Color	https://eonet.gsfc.nasa.gov/api/v2.1/categories/13	Related to events that alter the appearance of water: phytoplankton, red tide, algae, sediment, whiting, etc.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/13	\N	waterColor
14	Landslides	https://eonet.gsfc.nasa.gov/api/v2.1/categories/14	Related to landslides and variations thereof: mudslides, avalanche.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/14	\N	landslides
15	Sea and Lake Ice	https://eonet.gsfc.nasa.gov/api/v2.1/categories/15	Related to all ice that resides on oceans and lakes, including sea and lake ice (permanent and seasonal) and icebergs.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/15	\N	seaLakeIce
16	Earthquakes	https://eonet.gsfc.nasa.gov/api/v2.1/categories/16	Related to all manner of shaking and displacement. Certain aftermath of earthquakes may also be found under landslides and floods.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/16	\N	earthquakes
17	Snow	https://eonet.gsfc.nasa.gov/api/v2.1/categories/17	Related to snow events, particularly extreme/anomalous snowfall in either timing or extent/depth.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/17	\N	snow
18	Temperature Extremes	https://eonet.gsfc.nasa.gov/api/v2.1/categories/18	Related to anomalous land temperatures, either heat or cold.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/18	\N	temperaturesExtremes
19	Manmade	https://eonet.gsfc.nasa.gov/api/v2.1/categories/19	Events that have been human-induced and are extreme in their extent.	https://eonet.gsfc.nasa.gov/api/v2.1/layers/19	\N	manmade
\.

--
-- Name: category category_pkey; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.category
    ADD CONSTRAINT category_pkey PRIMARY KEY (id);


--
-- Name: eonet_calls eonet_calls_pkey; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.eonet_calls
    ADD CONSTRAINT eonet_calls_pkey PRIMARY KEY (id);


--
-- Name: event event_pkey; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.event
    ADD CONSTRAINT event_pkey PRIMARY KEY (id);


--
-- Name: quake fk_quake_code; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.quake
    ADD CONSTRAINT fk_quake_code PRIMARY KEY (code);


--
-- Name: event_source pk_event_source_id; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.event_source
    ADD CONSTRAINT pk_event_source_id PRIMARY KEY (source_id, event_id);


--
-- Name: source pk_source_id; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.source
    ADD CONSTRAINT pk_source_id PRIMARY KEY (id);


--
-- Name: geometry un_dt_coord_event; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.geometry
    ADD CONSTRAINT un_dt_coord_event UNIQUE NULLS NOT DISTINCT (dt, event_id) INCLUDE (coordinates);


--
-- Name: fire un_fire_instrument_dt_bright; Type: CONSTRAINT; Schema: public; Owner: hazards
--

ALTER TABLE ONLY public.fire
    ADD CONSTRAINT un_fire_instrument_dt_bright PRIMARY KEY (instrument, bright_ti4, "time");


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT ALL ON SCHEMA public TO hazards;


--
-- Name: TABLE category; Type: ACL; Schema: public; Owner: hazards
--

GRANT ALL ON TABLE public.category TO hazards;


--
-- Name: TABLE event; Type: ACL; Schema: public; Owner: hazards
--

GRANT ALL ON TABLE public.event TO hazards;


--
-- Name: TABLE event_source; Type: ACL; Schema: public; Owner: hazards
--

GRANT ALL ON TABLE public.event_source TO hazards;


--
-- Name: TABLE geometry; Type: ACL; Schema: public; Owner: hazards
--

GRANT ALL ON TABLE public.geometry TO hazards;


--
-- Name: SEQUENCE geometry_id_seq; Type: ACL; Schema: public; Owner: hazards
--

GRANT ALL ON SEQUENCE public.geometry_id_seq TO hazards;


--
-- Name: TABLE source; Type: ACL; Schema: public; Owner: hazards
--

GRANT ALL ON TABLE public.source TO hazards;

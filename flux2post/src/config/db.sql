SELECT 1;


CREATE TABLE IF NOT EXISTS event
(
  id varchar(255) PRIMARY KEY NOT NULL,
  title varchar(255) NOT NULL,
  description varchar(255) NULL,
  link varchar(255) NOT NULL,
  category_id smallint
);

CREATE TABLE category (
  id smallint PRIMARY KEY NOT NULL,
  title varchar(255),
  link varchar(255),
  description varchar(255),
  layers varchar(255)
);

ALTER TABLE event
    ADD CONSTRAINT  fk_event_category
    FOREIGN KEY (category_id)
    REFERENCES category(id);

CREATE TABLE source (
  id varchar(255)  NOT NULL,
  url varchar(255)  NOT NULL
);

CREATE TABLE event_source (
  source_id varchar(255),
  event_id varchar(255)
);

CREATE TABLE geometry (
  dt timestamp,
  type varchar(255),
  coordinates point NOT NULL
);

INSERT INTO category (id, title, link, description, layers)
values(6,'Drought','https://eonet.gsfc.nasa.gov/api/v2.1/categories/6','Long lasting absence of precipitation affecting agriculture and livestock, and the overall availability of food and water.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/6'),
(7,'Dust and Haze','https://eonet.gsfc.nasa.gov/api/v2.1/categories/7','Related to dust storms, air pollution and other non-volcanic aerosols. Volcano-related plumes shall be included with the originating eruption event.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/7'),
(16,'Earthquakes','https://eonet.gsfc.nasa.gov/api/v2.1/categories/16','Related to all manner of shaking and displacement. Certain aftermath of earthquakes may also be found under landslides and floods.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/16'),
(9,'Floods','https://eonet.gsfc.nasa.gov/api/v2.1/categories/9','Related to aspects of actual flooding--e.g., inundation, water extending beyond river and lake extents.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/9'),
(14,'Landslides','https://eonet.gsfc.nasa.gov/api/v2.1/categories/14','Related to landslides and variations thereof: mudslides, avalanche.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/14'),
(19,'Manmade','https://eonet.gsfc.nasa.gov/api/v2.1/categories/19','Events that have been human-induced and are extreme in their extent.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/19'),
(15,'Sea and Lake Ice','https://eonet.gsfc.nasa.gov/api/v2.1/categories/15','Related to all ice that resides on oceans and lakes, including sea and lake ice (permanent and seasonal) and icebergs.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/15'),
(10,'Severe Storms','https://eonet.gsfc.nasa.gov/api/v2.1/categories/10','Related to the atmospheric aspect of storms (hurricanes, cyclones, tornadoes, etc.). Results of storms may be included under floods, landslides, etc.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/10'),
(17,'Snow','https://eonet.gsfc.nasa.gov/api/v2.1/categories/17','Related to snow events, particularly extreme/anomalous snowfall in either timing or extent/depth.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/17'),
(18,'Temperature Extremes','https://eonet.gsfc.nasa.gov/api/v2.1/categories/18','Related to anomalous land temperatures, either heat or cold.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/18'),
(12,'Volcanoes','https://eonet.gsfc.nasa.gov/api/v2.1/categories/12','Related to both the physical effects of an eruption (rock, ash, lava) and the atmospheric (ash and gas plumes).','https://eonet.gsfc.nasa.gov/api/v2.1/layers/12'),
(13,'Water Color','https://eonet.gsfc.nasa.gov/api/v2.1/categories/13','Related to events that alter the appearance of water: phytoplankton, red tide, algae, sediment, whiting, etc.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/13'),
(8,'Wildfires','https://eonet.gsfc.nasa.gov/api/v2.1/categories/8','Wildland fires includes all nature of fire, in forest and plains, as well as those that spread to become urban and industrial fire events. Fires may be naturally caused or manmade.','https://eonet.gsfc.nasa.gov/api/v2.1/layers/8');


CREATE TABLE IF NOT EXISTS public.fire
(
  instrument  character varying(255),
  confidence  character varying(255),
  daynight  character varying(255),
  satellite  character varying(255),
  geometry point,
  bright_ti4 real,
  frp real,
  distance real,
  typ  character varying(255),
  time  timestamp without time zone
);
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.fire
    OWNER to hazards;
-- Table: public.quake

-- DROP TABLE IF EXISTS public.quake;

CREATE TABLE IF NOT EXISTS public.quake
(
    url character varying(255) COLLATE pg_catalog."default",
    alert character varying(255) COLLATE pg_catalog."default",
    code character varying(255) COLLATE pg_catalog."default" NOT NULL,
    magnitude real,
    geometry point,
    depth real,
    "time" timestamp without time zone,
    CONSTRAINT fk_quake_code PRIMARY KEY (code)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.quake
    OWNER to hazards;

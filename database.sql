CREATE TABLE temperatures (
    time_utc DATETIME NOT NULL, 
    time_local DATETIME NOT NULL, 
    temp_internal FLOAT NOT NULL, 
    temp_external FLOAT NOT NULL, 
    temp_owm FLOAT NOT NULL, 
    temp_owm_feels FLOAT NOT NULL, 
    condition VARCHAR NOT NULL, 
    PRIMARY KEY (time_utc)
);
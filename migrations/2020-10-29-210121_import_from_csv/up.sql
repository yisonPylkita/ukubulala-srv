-- Your SQL goes here
COPY homicides(Record_ID, Agency_Code, Agency_Name, Agency_Type, City, State, Year, Month, Incident, Crime_Type, Crime_Solved, Victim_Sex, Victim_Age, Victim_Race, Victim_Ethnicity, Perpetrator_Sex, Perpetrator_Age, Perpetrator_Race, Perpetrator_Ethnicity, Relationship, Weapon, Victim_Count, Perpetrator_Count, Record_Source)
FROM '/database.csv'
DELIMITER ','
CSV HEADER;

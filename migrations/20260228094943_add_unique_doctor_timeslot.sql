ALTER TABLE appointments 
ADD CONSTRAINT unique_doctor_timeslot 
UNIQUE (doctor_id, appointment_time);
-- Telomere Backend Initial Migration
-- MCOA Counter #2: Telomere Shortening Counter
-- Implements schema for D₂(n,t) kinetic equation storage

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Telomere measurements table (time-series data for D₂)
CREATE TABLE telomere_measurements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Subject identifier (links to LongevityCommon subject registry)
    subject_id UUID NOT NULL,
    
    -- Optional sample identifier for traceability
    sample_id VARCHAR(255),
    
    -- Measurement timestamp
    measured_at TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Core telomere metrics
    telomere_length_bp DECIMAL NOT NULL CHECK (telomere_length_bp >= 0),
    telomere_deficit_bp DECIMAL NOT NULL CHECK (telomere_deficit_bp >= 0),
    
    -- Kinetic equation variables
    population_doublings DECIMAL CHECK (population_doublings >= 0),
    time_elapsed_years DECIMAL CHECK (time_elapsed_years >= 0),
    
    -- Biological markers
    oxidative_stress_marker DECIMAL CHECK (oxidative_stress_marker >= 0),
    shelterin_expression DECIMAL CHECK (shelterin_expression >= 0),
    telomerase_activity DECIMAL CHECK (telomerase_activity >= 0),
    
    -- Measurement metadata
    measurement_method VARCHAR(255),
    metadata JSONB,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT unique_subject_sample_time UNIQUE (subject_id, sample_id, measured_at)
);

-- Telomere parameters table (stores D₂,₀, α₂, β₂, n₂*, τ₂, γ coefficients)
CREATE TABLE telomere_parameters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Subject identifier (UUID.nil for default parameters)
    subject_id UUID NOT NULL,
    
    -- Kinetic equation parameters (from PARAMETERS.md)
    d2_baseline DECIMAL NOT NULL CHECK (d2_baseline >= 0 AND d2_baseline <= 20000),
    alpha2 DECIMAL NOT NULL CHECK (alpha2 >= 0 AND alpha2 <= 500),
    beta2 DECIMAL NOT NULL CHECK (beta2 >= 0 AND beta2 <= 100),
    n2_star DECIMAL NOT NULL CHECK (n2_star >= 1 AND n2_star <= 200),
    tau2 DECIMAL NOT NULL CHECK (tau2 > 0),
    
    -- Coupling coefficients (γ) - all zero by default per CORRECTIONS_2026-04-22
    gamma_21 DECIMAL NOT NULL DEFAULT 0.0 CHECK (gamma_21 >= 0),
    gamma_23 DECIMAL NOT NULL DEFAULT 0.0 CHECK (gamma_23 >= 0),
    gamma_24 DECIMAL NOT NULL DEFAULT 0.0 CHECK (gamma_24 >= 0),
    gamma_25 DECIMAL NOT NULL DEFAULT 0.0 CHECK (gamma_25 >= 0),
    
    -- Metadata
    is_default BOOLEAN NOT NULL DEFAULT false,
    notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraints
    CONSTRAINT unique_subject_params UNIQUE (subject_id)
);

-- Create indexes for performance
CREATE INDEX idx_telomere_measurements_subject_id ON telomere_measurements(subject_id);
CREATE INDEX idx_telomere_measurements_measured_at ON telomere_measurements(measured_at);
CREATE INDEX idx_telomere_measurements_sample_id ON telomere_measurements(sample_id);
CREATE INDEX idx_telomere_parameters_subject_id ON telomere_parameters(subject_id);
CREATE INDEX idx_telomere_parameters_is_default ON telomere_parameters(is_default);

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_telomere_measurements_updated_at 
    BEFORE UPDATE ON telomere_measurements
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_telomere_parameters_updated_at 
    BEFORE UPDATE ON telomere_parameters
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default parameters from PARAMETERS.md
INSERT INTO telomere_parameters (
    id, subject_id, d2_baseline, alpha2, beta2, n2_star, tau2,
    gamma_21, gamma_23, gamma_24, gamma_25, is_default, notes
) VALUES (
    uuid_generate_v4(),
    '00000000-0000-0000-0000-000000000000', -- UUID.nil for default
    12500.0,  -- D₂,₀: 10-15kb range midpoint
    125.0,    -- α₂: 50-200 bp/PD midpoint
    35.0,     -- β₂: 20-50 bp/year midpoint
    50.0,     -- n₂*: 40-60 PD midpoint
    1.0,      -- τ₂: 1 year default
    0.0, 0.0, 0.0, 0.0,  -- All γ = 0 per CORRECTIONS_2026-04-22
    true,
    'Default parameters from PARAMETERS.md: D₂,₀=12500bp, α₂=125bp/PD, β₂=35bp/year, n₂*=50PD, τ₂=1yr, γ_i=0'
);
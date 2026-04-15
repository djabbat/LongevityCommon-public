export interface User {
  id: string
  username: string
  country_code?: string
  orcid_id?: string
  degree_verified: boolean
  fclc_node_active: boolean
  created_at: string
}

export type PsycheMood = 'very_good' | 'good' | 'neutral' | 'bad' | 'very_bad'

export interface HealthFactorSummary {
  /** Average psyche score [0,1] over last 30d */
  psyche?: number
  /** Average consciousness score [0,1] over last 30d */
  consciousness?: number
  /** Average social score [0,1] over last 30d */
  social?: number
  /** Integrated: 0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social */
  health_score?: number
  /** How many of the 4 factors have data (1–4) */
  factors_filled: number
}

export interface CreateHealthFactorRequest {
  recorded_at: string
  psyche_score?: number
  psyche_mood?: PsycheMood
  psyche_stress?: number
  psyche_notes?: string
  consciousness_score?: number
  consciousness_mindful?: number
  consciousness_purpose?: number
  consciousness_notes?: string
  social_score?: number
  social_support?: number
  social_isolation?: number
  social_notes?: string
}

export interface ZeProfile {
  user_id: string
  username: string
  chrono_age?: number
  // Organism (χ_Ze)
  bio_age_est?: number
  bio_age_ci_low?: number
  bio_age_ci_high?: number
  bio_age_delta?: number
  ci_stability?: 'high' | 'medium' | 'low'
  chi_ze_eeg?: number
  chi_ze_hrv?: number
  chi_ze_combined?: number
  trend_7d?: number
  trend_30d?: number
  fclc_node_active: boolean
  cohort_percentile?: number
  last_sample_at?: string
  sample_count: number
  // 4-Factor Health
  health_factors: HealthFactorSummary
}

export interface ZeTrendPoint {
  date: string
  chi_ze_combined?: number
  bio_age_est?: number
}

export interface ZeTrend {
  period_days: number
  points: ZeTrendPoint[]
}

export type PostType = 'ze_log' | 'science_thread' | 'study_invite' | 'debate'

export interface Post {
  id: string
  author_id: string
  type: PostType
  content: string
  doi?: string
  doi_verified: boolean
  code_url?: string
  data_url?: string
  score: number
  parent_id?: string
  study_id?: string
  created_at: string
  edited_at?: string
  author_username: string
  author_degree_verified: boolean
  reactions: {
    support: number
    replicate: number
    challenge: number
    cite: number
  }
}

export interface Study {
  id: string
  creator_id: string
  title: string
  hypothesis: string
  protocol: Record<string, unknown>
  target_n: number
  enrolled_n: number
  duration_days: number
  status: 'draft' | 'recruiting' | 'active' | 'completed' | 'published'
  result_doi?: string
  created_at: string
  starts_at?: string
  ends_at?: string
}

export interface ZeGuideResponse {
  session_id: string
  disclaimer: string
  response: string
  cited_dois: string[]
  cited_files: string[]
  model_used: string
}

export interface AuthResponse {
  token: string
  user: User
}

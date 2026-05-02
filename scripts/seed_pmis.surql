-- Seed file for PMIS Projects
DELETE project;

CREATE project:bridging_the_set_river SET
    project_code = 'BR-STR-2026',
    title = 'Seti River High-Tech Bridge Construction',
    description = 'A major infrastructure project aimed at connecting Gorkha Municipality with Sahid Lakhan Rural Municipality via a 200m signature bridge.',
    tier_level = 'federal',
    sector = 'infrastructure',
    implementation_modality = 'contractor',
    status = 'ongoing',
    planned_start_date = '2026-01-15T09:00:00Z',
    estimated_completion_date = '2028-12-30T17:00:00Z',
    estimated_total_cost = 450000000.0,
    location = {
        province: 'Gandaki',
        district: 'Gorkha',
        municipality: 'Gorkha Municipality',
        ward: 13,
        latitude: 28.0003,
        longitude: 84.5843,
        site_description: 'Near the historical bridge site at Seti River'
    },
    personnel = [
        {
            name: 'Er. Rajesh Kumar',
            designation: 'Senior Project Manager',
            organization: 'DoR',
            category: 'official',
            role: 'oversight'
        },
        {
            name: 'Er. Sita Thapa',
            designation: 'Site Engineer',
            organization: 'ABC Construction',
            category: 'contractor',
            role: 'supervisor'
        }
    ],
    financial_flows = [
        {
            amount: 150000000.0,
            fiscal_year: '2080/81',
            source: 'national_budget',
            flow_type: 'capital'
        },
        {
            amount: 5000000.0,
            fiscal_year: '2080/81',
            source: 'national_budget',
            flow_type: 'administrative'
        }
    ],
    results_framework = [
        {
            indicator: 'Foundation Piling Completion',
            target: 100.0,
            actual: 45.0,
            last_updated: '2081-01-10'
        },
        {
            indicator: 'Concrete Casting (Deck)',
            target: 100.0,
            actual: 0.0,
            last_updated: '2081-01-10'
        }
    ],
    communications_log = [
        {
            action: 'Site Inspection',
            actor_name: 'Er. Rajesh Kumar',
            timestamp: '2026-04-20T10:00:00Z',
            remarks: 'Foundation piling is progressing as scheduled. Safety measures are adequate.'
        },
        {
            action: 'Budget Disbursement',
            actor_name: 'MoF Admin',
            timestamp: '2026-04-15T14:00:00Z',
            remarks: 'First installment of capital budget released.'
        }
    ],
    opinions = [],
    reactions = [],
    created_at = '2026-04-26T10:14:00Z';

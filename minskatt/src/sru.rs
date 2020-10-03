use chrono::{Date, DateTime, Utc};
use std::fmt::Display;
use time::Time;

#[derive(Debug, Clone)]
pub enum SRU {
    Info(InfoHeader, InfoContact),
    Blanketter(Vec<Blankett>),
}

impl Display for SRU {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SRU::Info(header, contact) => fmt.write_str(&format!("{}{}", header, contact)),
            SRU::Blanketter(_) => fmt.write_str(""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InfoHeader {
    // #PRODUKT: this field is always "SRU"
    pub product: String,

    // #MEDIAID
    pub media_id: Option<String>,

    // #SKAPAD
    pub created_at: Option<DateTime<Utc>>,

    // #PROGRAM
    pub program: Option<String>,

    // #FILNAME
    pub filename: String,
}

impl Display for InfoHeader {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str("#DATABESKRIVNING_START\n")?;
        fmt.write_str("#PRODUKT SRU\n")?;
        if let Some(media_id) = &self.media_id {
            fmt.write_str(&format!("#MEDIAID {}\n", media_id))?;
        }
        if let Some(created_at) = &self.created_at {
            fmt.write_str(&format!("#SKAPAD {}\n", created_at.format("%Y%m%d %H%M%S")))?;
        }
        if let Some(program) = &self.program {
            fmt.write_str(&format!("#PROGRAM {}\n", program))?;
        }
        fmt.write_str(&format!("#FILNAMN {}\n", self.filename))?;
        fmt.write_str("#DATABESKRIVNING_SLUT\n")
    }
}

#[derive(Debug, Clone)]
pub struct InfoContact {
    // #ADRESS
    pub address: Option<String>,

    // #KONTACT
    pub contact_person: Option<String>,

    // #AVDELNING
    pub department: Option<String>,

    // #EMAIL
    pub email: Option<String>,

    // #FAX
    pub fax_number: Option<String>,

    // #NAMN
    pub name: String,

    // #ORGN
    pub organization_number: String,

    // #TELEFON
    pub phone_number: Option<String>,

    // #POSTNR
    pub zip_area: String,

    // #POSTORT
    pub zip_code: u32,
}

impl Display for InfoContact {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str("#MEDIELEV_START\n")?;
        fmt.write_str(&format!("#ORGNR {}\n", self.organization_number))?;
        fmt.write_str(&format!("#NAMN {}\n", self.name))?;
        if let Some(address) = &self.address {
            fmt.write_str(&format!("#ADRESS {}\n", address))?;
        }
        fmt.write_str(&format!("#POSTNR {}\n", self.zip_code))?;
        fmt.write_str(&format!("#POSTORT {}\n", self.zip_area))?;
        if let Some(department) = &self.department {
            fmt.write_str(&format!("#AVDELNING {}\n", department))?;
        }
        if let Some(contact_person) = &self.contact_person {
            fmt.write_str(&format!("#KONTAKT {}\n", contact_person))?;
        }
        if let Some(email) = &self.email {
            fmt.write_str(&format!("#EMAIL {}\n", email))?;
        }
        if let Some(phone_number) = &self.phone_number {
            fmt.write_str(&format!("#TELEFON {}\n", phone_number))?;
        }
        if let Some(fax_number) = &self.fax_number {
            fmt.write_str(&format!("#FAX {}\n", fax_number))?;
        }
        fmt.write_str("#MEDIELEV_SLUT\n")
    }
}
// @malinmnnikko note: line 120-121 + 133 are tied together because they depend on the document ID. These fields can be moved into a new type, that we create for a specific combination of these things.
// Dependents: which fields can be in the file
// Fill in the names of the field that INK3 can support for example
#[derive(Debug, Clone)]
pub enum Blankett {
    RawBlankett(RawBlankett),
    ValidatedBlankett(ValidatedBlankett),
}

#[derive(Debug, Clone)]
pub struct RawBlankett {
    // #BLANKETT
    document_id: String,
    document_appendix: Option<String>,
    filing_year: u8,
    filing_period: u8,

    // #IDENTITET
    organization_number: String,
    created_at: DateTime<Utc>,

    // #NAMN
    name: String,

    // #UPPGIFT
    fields: Vec<BlankettField>,

    // #SYSTEMINFO
    system_info: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidatedBlankett {
    // #BLANKETT
    document_id: String,
    document_appendix: Option<String>,
    filing_year: u8,
    filing_period: u8,

    // #IDENTITET
    organization_number: String,
    created_at: DateTime<Utc>,

    // #NAMN
    name: String,

    // #UPPGIFT
    document: ValidDocument,

    // #SYSTEMINFO
    system_info: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BlankettField {
    field_code: u16,
    field_value: String,
}

#[derive(Debug, Clone)]
pub enum ValidDocument {
    INK3S(INK3S2019P4),
}

#[derive(Debug, Clone)]

pub struct INK3S2019P4 {
    // INK3S = Skattemässiga justeringar Inkomstdeklaration 3

    // 7011 Räkenskapsårets början
    fiscal_year_start_date: Option<Date<Utc>>,
    // 7012 Räkenskapsårets slut
    fiscal_year_end_date: Option<Date<Utc>>,
    // 8686 7.1 Justerat resultat från föregående sida 6.21, vinst
    adjusted_result_from_previous_page_gains: Option<u32>,

    // 8786 7.2 Justerat resultat fr[n f;reg[ende sida, p. 6.21, förlust
    adjusted_result_from_previous_page_loses: Option<u32>,

    // 8787 7.3 Underskott a. Reduktion av underskott med hänsyn till exempelvis ackord
    deficit_a_unused_from_previous_year: Option<u32>,

    // 8687 7.3 Underskott b. Reduktion av underskott med hänsyn till exempelvis ackord
    deficit_b_unused_from_previous_year: Option<u32>,
    // 8788 7.4 Kostnader som ska dras av men som inte ing[r i det redovisade resultatet
    costs_that_shall_be_withdrawn_but_that_arent_included_in_the_audit: Option<u32>,

    // 8688 7.5a Intäkter som ska tas upp men som inte ingår i det redovisade resultatet: a. Beräknad schblonintäkt på kvarvarande periodiseringfonder vid beskattningsårets utgång
    estimated_income_at_the_end_of_the_fiscal_year_accural_fond: Option<u32>,

    // 8693 7.5b Intäkter som ska tas upp men som inte ingår i det redovisade resultatet: b. beräknad schablonintäkt på fondandelar ägda vid ingången av kalenderåtet
    estimated_income_at_the_beginning_of_the_calendar_year_fond_dividens: Option<u32>,

    // 8696 7.5c Intäkter som ska tas upp men som inte ingår i det redovisade resultatet: c. Uppräknat belopp vid återföring av periodeiseringsfond
    estimated_income_after_reentering_accural_fond: Option<u32>,

    // 8689 7.5d Intäkter som ska tas upp men som inte ingår i det redovisade resultatet: Övriga intäkter
    income_that_shall_be_declared_but_arent_in_the_declared_result_all_other_income: Option<u32>,

    // 8690 7.6 Skattemässig korrigering av bokfört resultat vid avyttring av fastighet och bostadsrätt
    taxation_correction_of_posted_results_in_connection_with_divestment_of_property_and_bostadsratt:
        Option<u32>,
    // 8789 7.6 samma som ovan, med med negativt resultat
    taxation_correction_of_posted_results_in_connection_with_divestment_of_property_and_bostadsratt_negative:
        Option<u32>,
    // 8790 7.7 Skogs-/substansminskningsavdrag (specificeras på blankett N8)
    forest_substance_reductions_decution: Option<u32>,
    // 8691 7.8 Återföring vid avyttring av fastighet (t.ex. värdeminskningsavdrag, skogsavdrag)
    reversal_in_connection_to_divestment_of_property: Option<u32>,
    //8695 7.9 Överskott (flyttas till huvudblanketten sid 1 p. 1.1) MOVE TO HEAD BLANKET POINT 1.1
    excess: Option<u32>,
    // 8795 7.10 Underskott (flyttas till huvudblanketten sid 1 p. 1.2) MOVE TO THE HEAD BLANKETT 1.2
    deficit: Option<u32>,
    // 8030 7.11 Vid beskattningsårets utgång ej återförda värdeminskingsavdrag avseende byggnader
    end_of_taxation_year_non_reversed_deprication_deduction_related_to_buildings: Option<u32>,
    // 8031 7.12 Vid beskattningsårets utgång ej återförda värdeminskningasvdrag avseende markanläggningar
    end_of_taxation_year_non_reversed_deprication_deduction_related_to_land_development:
        Option<u32>,
    // 8033 7.13 Vid restvärdeavskrivning: återförda belopp för av-och nedskrivning, försäljning. utrangering
    residual_value_depreciation_reversed_amounts_for_deprication_and_writedown_sale_scrapping:
        Option<u32>,
    // 8034 7.14 Avdrag för kapitalförvatlningskostnader har gjorts med
    deduction_for_captial_managementcosts_have_been_done_with: Option<u32>,
    // 8040/8041 Uppdragstagare har biträtt vid upprättandet av årsbokslutet/årsredovisningen: ja
    contractor_has_assisted_with_annual_accounts_reports: bool,

    // 8044/8045 Årsredovisningen har varit föremål för revision: ja
    annual_accounts_have_been_audited: bool,
}

pub struct INK3SU2019P4 {
    //Framställningsdatum
    creation_date: DateTime<Utc>,
    //framtställningstid
    creation_time: Time,
    //Fältkodsnummer
    field_code: Option<u32>,
    //Intern information för framställande program/system
    system_info: Option<String>,
    //korrekt organisationsummer
    correct_org_number: String,

    //Uppgiftslämnarens namn, no longer than 250 charachters
    information_giver_name: Option<String>,

    // 7011 räkenskapsårets början
    fiscal_year_start_date: Option<Date<Utc>>,
    // 7012 räkenskapsårets slut
    fiscal_year_end_date: Option<Date<Utc>>,
    // 8822 Föreningens/trossamfundets ändamål enligt stadarna - Idrott
    purpose_athletics: bool,
    //8828 Föreningens/trossamfundets ändamål enligt stadgarna - Kultur
    purpose_culture: bool,
    //8832 Föreningens/trossamfundets ändamål enligt stadgarna - Miljövård
    purpose_environmentalcare: bool,
    //8823 Föreningens/trossamfundets ändamål enligt stadgarna - Omsorg om barn och ungdom
    purpose_care_for_children_and_youths: bool,
    //8827 Föreningens/trossamfundets ändamål enligt stadgarna - Politisk verksamhet
    purpose_political_activities: bool,
    //8820 Föreningens/trossamfundets ändamål enligt stadgarna - Religiös verksamhet
    purpose_religious_activities: bool,
    // 8833 Föreningens/trossamfundets ändamål enligt stadgarna - Sjukvård
    purpose_healthcare: bool,
    // 8821 Föreningens/trossamfundets ändamål enligt stadgarna - social hjälpverksamhet
    purpose_social_aid: bool,
    // 8831 Föreningens/trossamfundets ändamål enligt stadgarna - Sveriges försvar och krisberedskap i samverkan med myndigheter
    purpose_Swedens_defense_in_collaboration_with_other_government_entities: bool,
    // 8829 Föreningens/trossamfundets ändamål enligt stadgarna - Utbildning
    purpose_education: bool,
    // 8830 Föreningens/trossamfundets ändamål enligt stadgarna - Vetenskaplig forskning
    purpose_scientific_research: bool,
    //8620 Föreningens/trossamfundets ändamål enligt stadgarna - annan likvärdig verksamhet, 36 charachter limit
    purpose_other_equvivalent: Option<String>,
    // 8614 Föreningar/trossamfund: hur stor del av fastighetens yta som hyrs ut, ange procenttal
    NGO_how_much_of_the_property_is_rented_percentage: Option<u32>,
    // 8615 Föreningar/trossamfund: hur stor del av fastigheten hyrs ut, ange i kvm
    NGO_how_much_of_the_property_is_rented_sqm: Option<u32>,
    // 8640 4.1 Föreningar/trossamfund: intäkter: medlemsavgifter
    NGO_income_membershipfees: Option<u32>,
    // 8641 4.2 Föreningar/trossamfund: intäkter : bidrag gåvor
    NGO_income_gifts_donations: Option<u32>,
    // 8642 4.3 Föreningar/trossamfund: intäkter : räntor, utdelningar
    NGO_income_interests_dividends: Option<u32>,
    // 8643 4.4 Föreningar/trossamfund: inktäkter: vinst vid försällning av fastigheter, värdepapper m-m
    NGO_income_profit_on_sale_of_property_securities_etc: Option<u32>,
    // 8629 4.5 Föreningar/trossamfund_ inktäkter: rörelseintäkter
    NGO_income_operational: Option<u32>,
    // 8644 4.6 Föreningar/trossamfund: intäkter: fastighetsintäkter
    NGO_income_property: Option<u32>,
    //8713 4.7 Föreningar/trossamfund: kostnader: utdelade bidrag, stipendier
    NGO_costs_membership_operations: Option<u32>,
    // 8714 4.8 Föreningar/trossamfund: kostnader: utdelade bidrag, stipendier
    NGO_costs_distributed_donations_grants: Option<u32>,
    // 8715 4.9 föreningar/trossamfund:kostnader: räntor och kapitalförvaltning
    NGO_costs_interestrates_capital_management: Option<u32>,
    // 8716 4.10 Föreningar trossamfund: kostnader: förlust vid försäljning av fastigheter, värdepapper m.ma
    NGO_costs_loss_when_selling_property: Option<u32>,
    // 8717 4.11 Föreningar/trossamfund: kostnader: rörelsekostnader
    NGO_costs_operational: Option<u32>,
    //8718 föreningar/trossamfund: kostnader : fastighetskostnader
    NGO_costs_propertycosts: Option<u32>,

    //Stiftelser
    //8840 Stiftelsens ändamål enligt stadgarna - annan likvärdig verksamhet, 36 charachter limit
    purpose_of_the_foundation_other_equvivalent_operation: Option<String>,
    //8850 3.1 Beslutad utdelning av bidrag/stipendier enligt ändamål
    decided_dividend_of_grants_according_to_purposes: Option<u32>,
    // 8857 3.2 Bidrag och gåvor
    contributions_gifts: Option<u32>,
    //8858 3.3 Kostnader för att erhålla bidrag och gåvor (insamlingskostnader)
    costs_for_recieving_contributions_collection_costs: Option<u32>,

    //8851 3.4 Kapitalavkastning
    return_on_capital: Option<u32>,
    //8852 3.5 Kapitalförvaltningskostnader
    asset_management_costs: Option<u32>,
    // 8853 3.6 Kostnader för att fullfölja stiftelsens ändamål
    costs_for_fullfilling_the_purpose_of_the_foundation: Option<u32>,
    //8854 3.7 Beslutad utdelning enligt p. 3.1 som jör till föregående års kapitalavkastning
    decided_divident_according_to_p_3_that_belongs_to_the_previous_years_capital_return: bool,
    // 8855 3.8 Uppgifter för verksamhetsstiftelser - intäkter
    information_for_operational_founds_income: Option<u32>,
    // 8856 3.9 Uppgifter för verksamhetsstiftelser - kostnader
    information_for_operational_founds_costs: Option<u32>,
}












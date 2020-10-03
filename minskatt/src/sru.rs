use chrono::{DateTime, Utc};
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
    fiscal_year_start_date: Option<DateTime<Utc>>,
    // 7012 Räkenskapsårets slut
    fiscal_year_end_date: Option<DateTime<Utc>>,
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
    //
}












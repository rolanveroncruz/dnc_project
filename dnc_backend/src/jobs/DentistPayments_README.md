# Dentist Payments #
Dentist Payments are payments made to the dentists. There are two components: retainer fee and per service fee.

###June 03, 2026 ###
We have a view, `unified_approved` that has the following columns:
- `source` - where did the record come from
- `id` - the id from the source
- `date_created` - the date the verification or accomplishment_reconciliation record was created
- `dentist_id` - the dentist
- `dentist_name` - the dentist
- `dental_clinic_id`
- `company_id`
- `company_name`
- `member_id`  
- `member_account_number`
- `member_name`         
- `dental_service_name`
- `date_service_performed`
- `tooth`                

## Retainer Fees ##
Rules:
1. Retainer fees are paid to dentists who are on "Flat Fee"(`dentist_contract.id=1`) contract. 
2. For each month that the dentist had made a service, he is paid the retainer fee. But he is paid only once.

Solution:
- Need table to track dentist services per month
  - June 04, 2026, created view `dentist_clinics_reconciled_jobs_count_last_12_months`
- Need table to track dentist payments.


## Per Service Fees ##


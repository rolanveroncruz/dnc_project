# HMO Billing #
The HMO Billing code is supposed to generate the HMO Billing report-spreadsheet tabulating the HMO Billing data.
My problem today is, the agreed generation is every 10th of the month. But with the pilot scheduled on June 1st, now
scheduled for July 3rd, we won't be able to test it without having to wait until the 10th. So instead, we'll have to
make it a manually triggered job.

HMO Billing is composed of two reports:
- The Company utilization report lists the patient name, treatment, teeth number, treatment date, and dentist name.
- The HMO Billing report just counts the number of billable patients.
  - if the company is billed annually, it counts all new patients input within in the month.
  - if the company is billed monthly, it counts all existing members.


# Status #
## June 2, 2026 ##
Currently, the utilization report is working. 
For the HMO Billing report:
1. api/hmo_billing.rs/get_generated_hmo_billing_reports lists the reports that have been generated.  
This is used by: `.route("/hmo_billing/", get(get_generated_hmo_billing_reports))`

2. api/hmo_billing.rs/download_generated_report  downloads a specific report.
This is used by: `.route("/hmo_billing/download/{file_name}", get(download_generated_report))`

3. `.route("/test_hmo_billing", get(test_generate_hmo_billing_reports))` calls 
`api/test_reports.rs/test_generate_hmo_biling()` which calls `jobs/hmo_billing.rs/generate_hmo_billing_reports()`

4. `jobs/hmo_billing.rs/generate_hmo_billing_reports()`:
   1. Generate BillingData for each endorsement.
        - for each endorsement:
            -- based on the billing type, count master_list_members count and count added_by_csrs count.
   2. Generate a Billing Report for each HMO.








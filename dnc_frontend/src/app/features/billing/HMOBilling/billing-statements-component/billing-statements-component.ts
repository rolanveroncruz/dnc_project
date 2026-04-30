import {Component, inject, OnInit, signal} from '@angular/core';
import {
    GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {MatButton} from '@angular/material/button';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatFormField, MatLabel} from '@angular/material/form-field';
import {MatIcon} from '@angular/material/icon';
import {MatOption} from '@angular/material/core';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {MatSelect} from '@angular/material/select';
import {HMO, HMOService} from '../../../../api_services/hmoservice';
import {HMOBillingService, HMOBillingRow} from '../../../../api_services/hmobilling-service';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';

@Component({
  selector: 'app-billing-statements-component',
    imports: [
        GenericDataTableComponent,
        MatButton,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle,
        MatFormField,
        MatIcon,
        MatLabel,
        MatOption,
        MatProgressSpinner,
        MatSelect
    ],
  templateUrl: './billing-statements-component.html',
  styleUrl: './billing-statements-component.scss',
})
export class BillingStatementsComponent implements OnInit {
    private hmoService = inject (HMOService);
    private hmoBillingService = inject(HMOBillingService);
    hmos = signal<HMO[]>([]);
    selected_hmo_id = signal<number | null>(null);
    selected_hmo_name = signal<string | null>(null);

    hmo_billing_report = signal<HMOBillingRow[]>([]);
    loading_hmos = signal<boolean>(false);

    HMOBillingReportColumns: TableColumn[] = [
        {key: 'statement_of_account_no', label: 'Statement of Account No.'},
        {key: 'company_name', label: 'Company Name'},
        {key: 'agreement_corp_number', label: 'Agreement Corp Number',},
        {key: 'total_master_list_members', label: 'Total'},
        {key: 'billing_period_type_name', label: 'Billing Period'},
        {key: 'dental_benefits', label: 'Benefits'},
        {key: 'retainer_fee', label: 'Fee'},
    ]

    ngOnInit(): void {
        this.loadHMOs();

    }
    loadHMOs(){
        this.loading_hmos.set(true);
        this.hmoService.getHMOs()
            .subscribe({
                next: (res) => {
                    this.hmos.set(res.items);
                    this.loading_hmos.set(false);
                },
                error: (err) => {
                    console.log("In load(), failed to load users", err);
                    this.loading_hmos.set(false);
                }
            });
    }

    onHmoSelected(hmoId: number) {
        this.selected_hmo_id.set(hmoId);

        // ✅ Set selected HMO name for the download filename
        const selectedHmo = this.hmos().find(hmo => hmo.id === hmoId);
        this.selected_hmo_name.set(selectedHmo?.short_name ?? null);

        this.load_billing_report_for_hmo(hmoId);
    }

    load_billing_report_for_hmo(hmoId:number){
        this.hmoBillingService.getHMOBillingForHMO(hmoId)
            .subscribe({
                next: (res) => {
                    this.hmo_billing_report.set(res);
                },
                error: (err) => {
                    console.log("In load_billing_report_for_hmo(), failed to load billing report", err);
                }
            });
    }
    downloadExcel() {
        const hmoId= this.selected_hmo_id();

        if (hmoId=== null) {
            return;
        }

        this.hmoBillingService
            .downloadBillingReportForHMO(hmoId)
            .subscribe({
                next: (blob) => {
                    const url = window.URL.createObjectURL(blob);

                    const a = document.createElement('a');
                    a.href = url;
                    a.download = `billing-report-${this.selected_hmo_name()}.xlsx`;
                    a.click();

                    window.URL.revokeObjectURL(url);
                },
                error: (err) => {
                    console.log('Failed to download utilization report', err);
                }
            });
    }
}

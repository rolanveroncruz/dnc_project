import {Component, inject, OnInit, signal} from '@angular/core';
import {HMOService, HMO, EndorsementCompanies} from '../../../../api_services/hmoservice';
import {MatCardModule} from '@angular/material/card';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatSelectModule} from '@angular/material/select';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {CommonModule} from '@angular/common';
import {FormsModule} from '@angular/forms';
import {UtilizationReportsService, UtilizationReportRow} from '../../../../api_services/utilization-reports-service';
import {
    GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {MatButton} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatInputModule} from '@angular/material/input';

@Component({
  selector: 'app-utilization-reports-component',
    imports: [
        CommonModule,
        FormsModule,
        MatCardModule,
        MatFormFieldModule,
        MatSelectModule,
        MatProgressSpinnerModule,
        GenericDataTableComponent,
        MatButton,
        MatIconModule,
        MatInputModule,
    ],
  templateUrl: './utilization-reports-component.html',
  styleUrl: './utilization-reports-component.scss',
})
export class UtilizationReportsComponent implements OnInit {
    private hmoService = inject (HMOService);
    private utilizationReportsService = inject(UtilizationReportsService);

    hmos = signal<HMO[]>([]);
    endorsement_companies = signal<EndorsementCompanies[]>([]);

    selected_hmo_id = signal<number | null>(null);
    selected_company_id = signal<number | null>(null);
    selected_company_name = signal<string | null>(null);
    start_date = signal<string | null>(null);
    end_date = signal<string | null>(null);

    loading_hmos = signal<boolean>(false);
    loading_companies = signal<boolean>(false);
    data = signal<UtilizationReportRow[]>([]);

    constructor() {

    }

    UtilizationReportColumns: TableColumn[] = [
        {key: 'member_account_number', label: 'Cert Number'},
        {key: 'member_name', label: 'Patient Name'},
        {key: 'dental_service_name', label: 'Treatment Done',},
        {key: 'tooth', label: 'Teeth Numbers'},
        {key: 'date_service_performed', label: 'Treatment Date', cellTemplateKey: 'date'},
        {key: 'dentist_name', label: 'Dentist'},
        {key: 'amount', label: 'Amount'},
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

        //  Reset company selection whenever HMO changes
        this.selected_company_id.set(null);
        this.selected_company_name.set(null);
        this.endorsement_companies.set([]);

        this.loadCompaniesForHmo(hmoId);
    }

    loadCompaniesForHmo(hmoId: number) {
        this.loading_companies.set(true);

        this.hmoService.getCompanies(hmoId)
            .subscribe({
                next: (res) => {
                    this.endorsement_companies.set(res);
                    this.loading_companies.set(false);
                },
                error: (err) => {
                    console.log('Failed to load endorsement companies', err);
                    this.loading_companies.set(false);
                }
            });
    }
    onStartDateChanged(value: string) {
        this.start_date.set(value || null); // ✅ added
        this.data.set([]); // ✅ clear stale report rows when the start date changes
    }

    onEndDateChanged(value: string) {
        this.end_date.set(value || null); // ✅ added
        this.data.set([]); // ✅ clear stale report rows when the end date changes
    }
    onCompanySelected(companyId: number) {
        this.selected_company_id.set(companyId);
        const selectedCompany = this.endorsement_companies()
            .find(company => company.id === companyId);

        this.selected_company_name.set(selectedCompany?.name ?? null);

        this.data.set([]);

        if (this.start_date() !== null || this.end_date() !== null) {
            this.getUtilizationReportData(companyId);
        }
    }

    loadReport(){
        const companyId = this.selected_company_id();
        if (companyId === null || this.start_date()===null || this.end_date()===null) {
            return;
        }
        this.getUtilizationReportData(companyId);
    }

    getUtilizationReportData(companyId: number){
        const startDate = this.start_date();
        const endDate = this.end_date();
        if (startDate === null || endDate === null) {
            return;
        }
        this.utilizationReportsService
            .getUtilizationReportForCompany(
                companyId,
                startDate,
                endDate,
            )
            .subscribe({
                next: (res) => {
                    console.log('Received utilization report data:', res);
                    this.data.set(res);
                },
                error: (err) => {
                    console.log('Failed to load utilization report data', err);
                }
            });
    }

    downloadExcel() {
        const companyId = this.selected_company_id();
        const startDate = this.start_date();
        const endDate = this.end_date();

        if (companyId === null || startDate === null || endDate === null) {
            return;
        }

        this.utilizationReportsService
            .downloadUtilizationReportForCompany(
                companyId,
                startDate,
                endDate,
            )
            .subscribe({
                next: (blob) => {
                    const url = window.URL.createObjectURL(blob);

                    const a = document.createElement('a');
                    a.href = url;
                    a.download = `utilization-report-${this.selected_company_name()}.xlsx`;
                    a.click();

                    window.URL.revokeObjectURL(url);
                },
                error: (err) => {
                    console.log('Failed to download utilization report', err);
                }
            });
    }
}

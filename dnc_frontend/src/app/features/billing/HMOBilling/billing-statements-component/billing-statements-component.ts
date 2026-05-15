import {Component, inject, OnInit, signal} from '@angular/core';
import {
    GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {MatButton} from '@angular/material/button';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {HMOBillingService, GeneratedBillingReportResponse} from '../../../../api_services/hmobilling-service';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';

@Component({
  selector: 'app-billing-statements-component',
    imports: [
        GenericDataTableComponent,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle,
    ],
  templateUrl: './billing-statements-component.html',
  styleUrl: './billing-statements-component.scss',
})
export class BillingStatementsComponent implements OnInit {
    private hmoBillingService = inject(HMOBillingService);

    hmo_billing_reports = signal<GeneratedBillingReportResponse[]>([]);

    HMOBillingReportColumns: TableColumn[] = [
        {key: 'id', label: 'Id'},
        {key: 'file_name', label: 'Name'},
        {key: 'date_generated', label: 'Date Generated', cellTemplateKey: 'date'},
    ]

    ngOnInit(): void {
        this.loadGeneratedHMOBillingReports();

    }
    loadGeneratedHMOBillingReports(){
        this.hmoBillingService.getHMOBillingReports()
            .subscribe({
                next: (res) => {
                    this.hmo_billing_reports.set(res);
                },
                error: (err) => {
                    console.log("In load(), failed to load users", err);
                }
            });
    }
    downloadReport(row: GeneratedBillingReportResponse) {
        this.hmoBillingService.downloadGeneratedReport(row.file_name)
            .subscribe({
                next: (blob) => {
                    const url = window.URL.createObjectURL(blob);

                    const a = document.createElement('a');
                    a.href = url;
                    a.download = row.file_name;
                    a.click();

                    window.URL.revokeObjectURL(url);
                },
                error: (err) => {
                    console.log('Failed to download report', err);
                }
            });
    }


}

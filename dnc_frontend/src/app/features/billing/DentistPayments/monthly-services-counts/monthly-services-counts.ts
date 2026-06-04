import {Component, inject, OnInit, signal} from '@angular/core';
import {DentistRetainerFeesService, DentistClinicReconciledJobsPivotRow} from '../../../../api_services/dentist-retainer-fees-service';
import type {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {
    GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';

@Component({
  selector: 'app-monthly-services-counts',
    imports: [
        GenericDataTableComponent,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardSubtitle,
        MatCardTitle
    ],
  templateUrl: './monthly-services-counts.html',
  styleUrl: './monthly-services-counts.scss',
})
export class MonthlyServicesCounts implements OnInit{
    private readonly dentistRetainerFeesService = inject(DentistRetainerFeesService);
    data = signal<DentistClinicReconciledJobsPivotRow[]>([]);

    columns = signal<TableColumn<DentistClinicReconciledJobsPivotRow>[]>([
        { key: 'dentist_name', label: 'Dentist' },
        { key: 'clinic_name', label: 'Clinic' },
    ]);

    private readonly fixedKeys = new Set<string>([
        'dentist_name',
        'clinic_name',
        'contract_name',
        'id',
        'position_name',
    ])

    ngOnInit(){
        this.loadData();
    }
    loadData(){
        this.dentistRetainerFeesService.getDentistClinicReconciledJobsLast12Months()
        .subscribe({
            next: (data) => {
                console.log('In loadData(), data:', data);
                const dynamicMonthColumns = this.getMonthColumns(data);
                this.columns.set([
                    {key: 'dentist_name', label: 'Dentist'},
                    {key: 'clinic_name', label: 'Clinic'},
                    ...dynamicMonthColumns,
                ])

                this.data.set(data);
            },
            error: (err) => {
                console.log('In loadData(), failed to load data', err);
            },
        });
    }
    private getMonthColumns(
        rows: DentistClinicReconciledJobsPivotRow[]
    ): TableColumn<DentistClinicReconciledJobsPivotRow>[] {

        if (rows.length === 0) {
            return [];
        }
        const firstRow = rows[0];
        return Object.keys(firstRow)
            .filter(key=>!this.fixedKeys.has(key))
            .map( key=>({
                key,
                label:key,
            }));
    }
}

import {Component, inject, OnInit, signal} from '@angular/core';
import {HMOService, HMO, EndorsementCompanies} from '../../../../api_services/hmoservice';
import {chmodSync} from 'node:fs';
import {MatCardModule} from '@angular/material/card';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatSelectModule} from '@angular/material/select';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {CommonModule} from '@angular/common';
import {FormsModule} from '@angular/forms';

@Component({
  selector: 'app-utilization-reports-component',
  imports: [
      CommonModule,
      FormsModule,
      MatCardModule,
      MatFormFieldModule,
      MatSelectModule,
      MatProgressSpinnerModule,
  ],
  templateUrl: './utilization-reports-component.html',
  styleUrl: './utilization-reports-component.scss',
})
export class UtilizationReportsComponent implements OnInit {
    private hmoService = inject (HMOService);

    hmos = signal<HMO[]>([]);
    endorsement_companies = signal<EndorsementCompanies[]>([]);

    selected_hmo_id = signal<number | null>(null);
    selected_company_id = signal<number | null>(null);
    loading_hmos = signal<boolean>(false);
    loading_companies = signal<boolean>(false);

    constructor() {

    }

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

    onCompanySelected(companyId: number) {
        this.selected_company_id.set(companyId);

        // Later, this is where you will load the next data from the server.
        console.log('Selected company_id:', companyId);
    }
}

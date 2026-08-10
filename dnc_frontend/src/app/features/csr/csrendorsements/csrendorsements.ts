import { Component, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';

import { MatCardModule } from '@angular/material/card';

import { CSREndorsementsService, CsrEndorsementResponse } from './csrendorsements-service';

// ✅ Adjust these paths to where your GenericDataTableComponent is located
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';

@Component({
    selector: 'app-csrendorsements',
    standalone: true,
    imports: [
        CommonModule,
        MatCardModule,
        GenericDataTableComponent,
    ],
    templateUrl: './csrendorsements.html',
    styleUrl: './csrendorsements.scss',
})
export class CSREndorsements implements OnInit {

    private readonly csrEndorsementsService = inject(CSREndorsementsService);

    endorsements = signal<CsrEndorsementResponse[]>([]);
    loading = signal(false);
    errorMessage = signal<string | null>(null);

    columns: TableColumn[] = [
        {
            key: 'hmo_short_name',
            label: 'HMO',
        },
        {
            key: 'company_name',
            label: 'Company',
        },
        {
            key: 'date_start',
            label: 'Start Date',
            cellTemplateKey: 'date',
        },
        {
            key: 'date_end',
            label: 'End Date',
            cellTemplateKey: 'date',
        },
        {
            key: 'benefits',
            label: 'Benefits',
        },
    ];

    ngOnInit(): void {
        this.loadEndorsements();
    }

    loadEndorsements(): void {
        this.loading.set(true);
        this.errorMessage.set(null);

        this.csrEndorsementsService.getEndorsements().subscribe({
            next: (endorsements) => {
                this.endorsements.set(endorsements);
                this.loading.set(false);
            },
            error: (error) => {
                console.error('Failed to load CSR endorsements:', error);
                this.errorMessage.set('Failed to load endorsements.');
                this.loading.set(false);
            },
        });
    }
}

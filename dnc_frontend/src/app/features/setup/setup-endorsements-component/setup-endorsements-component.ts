import { CommonModule } from '@angular/common';
import {Component, ChangeDetectionStrategy, inject, signal, OnInit} from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import {HMOService} from '../../../api_services/hmoservice';

import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatRadioModule } from '@angular/material/radio';
import { MatTabsModule } from '@angular/material/tabs';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatNativeDateModule } from '@angular/material/core';

type BillingFrequency = 'annual' | 'monthly';

@Component({
    selector: 'app-setup-endorsements',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [
        CommonModule,
        ReactiveFormsModule,

        MatCardModule,
        MatFormFieldModule,
        MatInputModule,
        MatSelectModule,
        MatRadioModule,
        MatTabsModule,
        MatDatepickerModule,
        MatNativeDateModule,
    ],
    templateUrl: './setup-endorsements-component.html',
    styleUrls: ['./setup-endorsements-component.scss'],
})
export class SetupEndorsementsComponent implements OnInit{
    ngOnInit(): void {

    }
    private readonly fb = inject(FormBuilder);

    // Replace these with API-driven options later.
    readonly hmoOptions = signal<string[]>(['Maxicare', 'Medicard', 'Intellicare', 'PhilCare']);
    readonly companyOptions = signal<string[]>(['Acme Corp', 'Globex', 'Initech', 'Umbrella']);
    readonly endorsementTypeOptions = signal<string[]>([
        'New Coverage',
        'Renewal',
        'Upgrade',
        'Downgrade',
        'Termination',
        'Addendum',
    ]);
    readonly endorsementMethodOptions = signal<string[]>(['Email', 'Portal', 'Courier', 'In-Person']);

    readonly form = this.fb.group({
        // Column 1
        hmo: ['', [Validators.required]],
        company: ['', [Validators.required]],
        agreementNumber: ['', Validators.required],

        // Column 2
        billingFrequency: ['annual' as BillingFrequency, [Validators.required]],
        retainerFee: [null as number | null, [Validators.min(0)]],
        dateStart: [null as Date | null],
        dateEnd: [null as Date | null],

        // Column 3
        endorsementType: [null as string | null, [Validators.required]],
        remarks: [''],
        endorsementMethod: [null as string | null],
    });
}

import {Component, inject, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {HttpClient} from '@angular/common/http';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCard} from '@angular/material/card';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {MatFormField, MatLabel, MatError} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatSelect, MatOption} from '@angular/material/select';
import {environment} from '../../../environments/environment';

type ContactPersonType = 'member' | 'dentist' | 'broker' | 'hmo_rep';

interface SubmitContactUsMessageRequest {
    person_type: ContactPersonType;
    name: string;
    card_number: string | null;
    company_and_hmo: string | null;
    contact_numbers: string;
    message: string;
}

interface SubmitContactUsMessageResponse {
    id: number;
    message: string;
}

@Component({
    selector: 'app-contact-us',
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatCard,
        MatButton,
        MatIcon,
        MatFormField,
        MatLabel,
        MatError,
        MatInput,
        MatSelect,
        MatOption,
    ],
    templateUrl: './contact-us-component.html',
    styleUrl: './contact-us-component.scss',
})
export class ContactUsComponent {
    private readonly http = inject(HttpClient);
    private readonly baseUrl = environment.apiUrl;

    readonly isSubmitted = signal(false);
    readonly isSubmitting = signal(false);
    readonly submitError = signal<string | null>(null);

    readonly personTypes: { value: ContactPersonType; label: string }[] = [
        { value: 'member', label: 'I’m a member' },
        { value: 'dentist', label: 'I’m a dentist' },
        { value: 'broker', label: 'I’m a broker' },
        { value: 'hmo_rep', label: 'I’m an HMO representative' },
    ];

    readonly form = new FormGroup({
        person_type: new FormControl<ContactPersonType | null>(null, {
            validators: [Validators.required],
        }),
        name: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
        card_number: new FormControl('', {
            nonNullable: true,
        }),
        company_and_hmo: new FormControl('', {
            nonNullable: true,
        }),
        contact_numbers: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
        message: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
    });

    get isMember(): boolean {
        return this.form.controls.person_type.value === 'member';
    }

    get canSubmit(): boolean {
        return this.form.valid && !this.isSubmitted() && !this.isSubmitting();
    }

    submitMessage(): void {
        if (!this.canSubmit) {
            this.form.markAllAsTouched();
            return;
        }

        const rawValue = this.form.getRawValue();

        if (!rawValue.person_type) {
            this.form.controls.person_type.markAsTouched();
            return;
        }

        const request: SubmitContactUsMessageRequest = {
            person_type: rawValue.person_type,
            name: rawValue.name.trim(),
            card_number: rawValue.card_number.trim() || null,
            company_and_hmo: rawValue.company_and_hmo.trim() || null,
            contact_numbers: rawValue.contact_numbers.trim(),
            message: rawValue.message.trim(),
        };

        this.submitError.set(null);
        this.isSubmitting.set(true);

        this.http.post<SubmitContactUsMessageResponse>(
            `${this.baseUrl}/public/contact_messages`,
            request
        ).subscribe({
            next: () => {
                this.isSubmitting.set(false);
                this.isSubmitted.set(true);
                this.form.disable();
            },
            error: (error) => {
                console.error('Error submitting contact us message:', error);

                this.isSubmitting.set(false);
                this.submitError.set(
                    'Sorry, your message could not be submitted. Please try again.'
                );
            },
        });
    }
}

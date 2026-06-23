import {Component, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCard} from '@angular/material/card';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {MatFormField, MatLabel, MatError} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatSelect, MatOption} from '@angular/material/select';

type ContactPersonType = 'member' | 'dentist' | 'broker' | 'hmo_rep';

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
    readonly isSubmitted = signal(false);

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
        return this.form.valid && !this.isSubmitted();
    }

    submitMessage(): void {
        if (!this.canSubmit) {
            this.form.markAllAsTouched();
            return;
        }

        // Temporary frontend-only behavior.
        // Later, replace this with a backend service call.
        this.isSubmitted.set(true);
        this.form.disable();
    }
}

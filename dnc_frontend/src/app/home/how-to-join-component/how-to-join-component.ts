import {Component,inject, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCard} from '@angular/material/card';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {MatFormField,  MatLabel, MatError} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {HowToJoinService} from './how-to-join-service';

@Component({
    selector: 'app-how-to-join',
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatCard,
        MatButton,
        MatIcon,
        MatFormField,
        MatInput,
        MatLabel,
        MatError,
        MatProgressSpinner,
    ],
    templateUrl: './how-to-join-component.html',
    styleUrl: './how-to-join-component.scss',
})
export class HowToJoinComponent {
    private readonly howToJoinService = inject(HowToJoinService);

    readonly prcLicenseFile = signal<File | null>(null);
    readonly bir2303File = signal<File | null>(null);

    readonly isSubmitting = signal(false);
    readonly isSubmitted = signal(false);
    readonly submitError = signal<string | null>(null);

    readonly form = new FormGroup({
        name: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
        clinic_name: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
        contact_numbers: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
        email: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required, Validators.email],
        }),
    });

    get canSubmit(): boolean {
        return this.form.valid &&
            this.prcLicenseFile() !== null &&
            this.bir2303File() !== null &&
            !this.isSubmitting() &&
            !this.isSubmitted();
    }

    onPrcLicenseSelected(event: Event): void {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0] ?? null;
        this.prcLicenseFile.set(file);
    }

    onBir2303Selected(event: Event): void {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0] ?? null;
        this.bir2303File.set(file);
    }

    clearPrcLicense(): void {
        if (this.isSubmitted()) {
            return;
        }

        this.prcLicenseFile.set(null);
    }

    clearBir2303(): void {
        if (this.isSubmitted()) {
            return;
        }

        this.bir2303File.set(null);
    }

    submitApplication(): void {
        if (!this.canSubmit) {
            this.form.markAllAsTouched();
            return;
        }
        const prcLicenseFile = this.prcLicenseFile();
        const bir2303File = this.bir2303File();

        if (!prcLicenseFile || !bir2303File) {
            return;
        }

        const rawValue = this.form.getRawValue();

        const formData = new FormData();
        formData.append('name', rawValue.name);
        formData.append('clinic_name', rawValue.clinic_name);
        formData.append('contact_numbers', rawValue.contact_numbers);
        formData.append('email', rawValue.email);
        formData.append('prc_license_file', prcLicenseFile);
        formData.append('bir_2303_file', bir2303File);

        this.submitError.set(null);
        this.isSubmitting.set(true);

        this.howToJoinService.submitDentistApplication(formData).subscribe({
            next: () => {
                this.isSubmitting.set(false);
                this.isSubmitted.set(true);
                this.form.disable();
            },
            error: (error) => {
                console.error('Error submitting application:', error);
                this.isSubmitting.set(false);
                this.submitError.set("Sorry, your application could not be submitted. Please try again.");
            },
        });
    }

}

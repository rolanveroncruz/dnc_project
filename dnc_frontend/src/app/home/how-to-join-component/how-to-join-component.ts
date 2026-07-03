import {Component,DestroyRef, inject, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCardModule} from '@angular/material/card';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatFormFieldModule,  MatLabel, MatError} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {
    ClinicOwnershipType,
    DentistApplicationFormValue,
    HowToJoinService,
} from './how-to-join-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatOption} from '@angular/material/core';
import {MatSelectModule} from '@angular/material/select';

@Component({
    selector: 'app-how-to-join',
    imports: [
        CommonModule,
        ReactiveFormsModule,
        MatCardModule,
        MatButtonModule,
        MatIconModule,
        MatFormFieldModule,
        MatInputModule,
        MatLabel,
        MatError,
        MatProgressSpinnerModule,
        MatOption,
        MatSelectModule,
    ],
    templateUrl: './how-to-join-component.html',
    styleUrl: './how-to-join-component.scss',
})
export class HowToJoinComponent {
    private readonly howToJoinService = inject(HowToJoinService);
    private readonly DestroyRef = inject(DestroyRef);

    readonly prcLicenseFile = signal<File | null>(null);
    readonly bir2303File = signal<File | null>(null);
    readonly registrationDocFile = signal<File | null>(null);
    readonly supportingDocsFile1 = signal<File | null>(null);

    readonly isSubmitting = signal(false);
    readonly isSubmitted = signal(false);
    readonly submitError = signal<string | null>(null);
    readonly submitAttempted = signal(false);

    readonly clinicOwnershipOptions: { value: ClinicOwnershipType; label: string }[] = [
        {
            value: 'single_proprietorship',
            label: 'Single Proprietorship',
        },
        {
            value: 'company',
            label: 'Company',
        },
        {
            value: 'corporation',
            label: 'Corporation',
        },
    ];

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

        clinic_ownership_type: new FormControl<ClinicOwnershipType | null>(null, {
            validators: [Validators.required],
        }),

        hmo_affiliations: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),

        clinic_address: new FormControl('', {
            nonNullable: true,
            validators: [Validators.required],
        }),
    });

    constructor() {
        this.form.controls.clinic_ownership_type.valueChanges
            .pipe(takeUntilDestroyed(this.DestroyRef))
            .subscribe((ownershipType) => {
                if (ownershipType === 'single_proprietorship') {
                    this.supportingDocsFile1.set(null);
                }
            })
    }
    get selectedOwnershipType(): ClinicOwnershipType | null {
        return this.form.controls.clinic_ownership_type.value;
    }

    get registrationDocLabel(): string {
        if (this.selectedOwnershipType === 'single_proprietorship') {
            return 'DTI Registration';
        }

        if (
            this.selectedOwnershipType === 'company' ||
            this.selectedOwnershipType === 'corporation'
        ) {
            return 'SEC Registration';
        }

        return 'Registration Document';
    }

    get supportingDocsRequired(): boolean {
        return (
            this.selectedOwnershipType === 'company' ||
            this.selectedOwnershipType === 'corporation'
        );
    }



    get canSubmit(): boolean {
        const hasRequiredFiles =
            this.prcLicenseFile() !== null &&
            this.bir2303File() !== null &&
            this.registrationDocFile() !== null;

        const hasConditionalSupportingDocs =
            !this.supportingDocsRequired || this.supportingDocsFile1() !== null;

        return (
            this.form.valid &&
            hasRequiredFiles &&
            hasConditionalSupportingDocs &&
            !this.isSubmitting() &&
            !this.isSubmitted()
        );
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

    onRegistrationDocSelected(event: Event): void {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0] ?? null;
        this.registrationDocFile.set(file);
    }

    onSupportingDocsFile1Selected(event: Event): void {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0] ?? null;
        this.supportingDocsFile1.set(file);
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

    clearRegistrationDoc(): void {
        if (this.isSubmitted()) {
            return;
        }

        this.registrationDocFile.set(null);
    }

    clearSupportingDocsFile1(): void {
        if (this.isSubmitted()) {
            return;
        }

        this.supportingDocsFile1.set(null);
    }

    submitApplication(): void {
        this.submitAttempted.set(true);

        if (!this.canSubmit) {
            this.form.markAllAsTouched();
            return;
        }

        const rawValue = this.form.getRawValue();
        const ownershipType = this.selectedOwnershipType;
        const prcLicenseFile = this.prcLicenseFile();
        const bir2303File = this.bir2303File();
        const registrationDocFile = this.registrationDocFile();
        const supportingDocsFile1 = this.supportingDocsRequired
            ?this.supportingDocsFile1()
            :null;

        if (
            !ownershipType ||
            !prcLicenseFile ||
            !bir2303File ||
            !registrationDocFile ||
            (this.supportingDocsRequired && !supportingDocsFile1)
        ) {
            return;
        }

        const formValue: DentistApplicationFormValue = {
            name: rawValue.name,
            clinic_name: rawValue.clinic_name,
            contact_numbers: rawValue.contact_numbers,
            email: rawValue.email,
            clinic_ownership_type: ownershipType,
            hmo_affiliations: rawValue.hmo_affiliations,
            clinic_address: rawValue.clinic_address,
        };

        this.submitError.set(null);
        this.isSubmitting.set(true);

        this.howToJoinService.submitDentistApplication(formValue,{
            prcLicenseFile,
            bir2303File,
            registrationDocFile,
            supportingDocsFile1,
        }).subscribe({
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

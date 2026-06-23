import {Component, inject, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormControl, ReactiveFormsModule, Validators} from '@angular/forms';
import {MatCard} from '@angular/material/card';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {MatFormField, MatLabel, MatError} from '@angular/material/form-field';
import {MatInput} from '@angular/material/input';
import {MatProgressSpinner} from '@angular/material/progress-spinner';
import {
    FindDentistService,
    PublicDentistSearchResult
} from './find-dentist-service';

@Component({
    selector: 'app-find-dentist',
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
        MatProgressSpinner,
    ],
    templateUrl: './find-dentist-component.html',
    styleUrl: './find-dentist-component.scss',
})
export class FindDentistComponent {
    private readonly findDentistService = inject(FindDentistService);

    readonly searchControl = new FormControl('', {
        nonNullable: true,
        validators: [Validators.required],
    });

    readonly results = signal<PublicDentistSearchResult[]>([]);
    readonly hasSearched = signal(false);
    readonly isSearching = signal(false);
    readonly searchError = signal<string | null>(null);

    searchDentists(): void {
        if (this.searchControl.invalid) {
            this.searchControl.markAsTouched();
            return;
        }

        const query = this.searchControl.value.trim();

        if (!query) {
            this.searchControl.markAsTouched();
            return;
        }

        this.hasSearched.set(true);
        this.isSearching.set(true);
        this.searchError.set(null);
        this.results.set([]);

        this.findDentistService.searchDentists(query).subscribe({
            next: (results) => {
                this.results.set(results);
                this.isSearching.set(false);
            },
            error: (error) => {
                console.error('Error searching dentists:', error);
                this.isSearching.set(false);
                this.searchError.set(
                    'Sorry, we could not complete your search. Please try again.'
                );
            },
        });
    }

    clearSearch(): void {
        this.searchControl.setValue('');
        this.results.set([]);
        this.hasSearched.set(false);
        this.isSearching.set(false);
        this.searchError.set(null);
    }

    get fullAddressLabel(): string {
        return 'Address, City, or Region';
    }
}

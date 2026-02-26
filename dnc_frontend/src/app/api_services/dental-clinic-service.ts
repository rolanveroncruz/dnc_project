// src/app/api_services/dental-clinic-service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import {catchError, Observable, throwError, map, tap} from 'rxjs';

import { environment } from '../../environments/environment';
import { LoginService } from '../login.service';

//
// ---- Shared paging types (unchanged)
//

export interface ListQuery {
    page?: number;       // 1-based
    page_size?: number;  // server clamps
}

export interface PageResponse<T> {
    page: number;       // 1-based
    page_size: number;
    total: number;
    items: T[];
}

//
// ---- Handler-aligned response types
//

// ✅ CHANGED: This is the base "row" returned by your SELECT into DentalClinicRowDb
export interface DentalClinicRowDb {
    // dental_clinic columns
    id: number;
    name: string;
    owner_name: string | null;            // ✅ CHANGED: include owner_name (was missing)
    address: string;
    city_id: number | null;
    zip_code: string | null;
    remarks: string | null;
    contact_numbers: string | null;
    email: string | null;
    schedule: string | null;
    active: boolean | null;
    last_modified_by: string;
    last_modified_on: string; // DateTimeWithTimeZone serialized

    // joined fields
    city_name: string | null;
    province_id: number | null;
    province_name: string | null;
    region_id: number | null;
    region_name: string | null;

    // ---- accounting fields (from dental_clinic table)
    acct_tin: string | null;
    acct_bank_name: string | null;
    acct_account_type: number | null;
    acct_account_name: string | null;
    acct_account_number: string | null;
    acct_tax_type: number | null;
    acct_tax_classification: number | null;
    acct_trade_name: string | null;
    acct_taxpayer_name: string | null;

    // ---- accounting lookup names (joined)
    acct_account_type_name: string | null;
    acct_tax_type_name: string | null;
    acct_tax_classification_name: string | null;
}

// ✅ NEW: This matches your API list item DentalClinicRow (RowDb + capability flags)
export interface DentalClinicRow extends DentalClinicRowDb {
    // handler uses serde rename: hasPanoramic / hasPeriapical
    hasPanoramic: boolean;
    hasPeriapical: boolean;
}

// ✅ NEW: This matches what SeaORM insert/update returns (dental_clinic::Model).
// It will NOT include city/province/region names nor capability flags.
export interface DentalClinicModel {
    id: number;
    name: string;
    owner_name: string | null;            // ✅ include owner_name
    address: string;
    city_id: number | null;
    zip_code: string | null;
    remarks: string | null;
    contact_numbers: string | null;
    email: string | null;
    schedule: string | null;

    // ✅ CHANGE: include accounting fields on model, because backend Model has them
    acct_tin: string | null;
    acct_bank_name: string | null;
    acct_account_type: number | null;
    acct_account_name: string | null;
    acct_account_number: string | null;
    acct_tax_type: number | null;
    acct_tax_classification: number | null;
    acct_trade_name: string | null;
    acct_taxpayer_name: string | null;

    active: boolean | null;
    last_modified_by: string;
    last_modified_on: string;
}

export interface DentalClinicListQuery extends ListQuery {
    city_id?: number;
    active?: boolean;
    name_like?: string;
}

//
// ---- Request body types (must mirror Rust)
//

// ✅ CHANGED: create body must match Rust CreateDentalClinicBody exactly.
// Removed city/province/region names + hasPanoramic/hasPeriapical (those are not accepted by POST).
export interface CreateDentalClinicBody {
    name: string;
    address: string;

    owner_name?: string | null;           // ✅ CHANGED: add (Rust has owner_name: Option<String>)
    city_id?: number | null;
    zip_code?: string | null;
    remarks?: string | null;
    contact_numbers?: string | null;
    email?: string | null;
    schedule?: string | null;

    // ✅ CHANGE: add accounting fields
    acct_tin?: string | null;
    acct_bank_name?: string | null;
    acct_account_type?: number | null;
    acct_account_name?: string | null;
    acct_account_number?: string | null;
    acct_tax_type?: number | null;
    acct_tax_classification?: number | null;
    acct_trade_name?: string | null;
    acct_taxpayer_name?: string | null;
    active?: boolean | null;

    last_modified_by: string; // required by your Rust API
}

/**
 * PATCH semantics (matches Rust Option<Option<T>> fields):
 * - Omit field => None => don't change
 * - Include field as null => Some(None) => explicitly set NULL
 * - Include field as value => Some(Some(value)) => set to that value
 */

export interface PatchDentalClinicBody {
    name?: string;
    address?: string;

    // ✅ CHANGED: Rust PATCH includes owner_name: Option<Option<String>>
    owner_name?: string | null;

    // ✅ NOTE: these are nullable fields in Rust PATCH via Option<Option<T>>
    city_id?: number | null;
    zip_code?: string | null;
    remarks?: string | null;
    contact_numbers?: string | null;
    email?: string | null;
    schedule?: string | null;

    // ✅ CHANGE: add accounting fields
    acct_tin?: string | null;
    acct_bank_name?: string | null;
    acct_account_type?: number | null;
    acct_account_name?: string | null;
    acct_account_number?: string | null;
    acct_tax_type?: number | null;
    acct_tax_classification?: number | null;
    acct_trade_name?: string | null;
    acct_taxpayer_name?: string | null;

    active?: boolean | null;

    last_modified_by: string; // required
}


export interface ClinicWithCapabilities {
    // clinic fields
    id: number;
    name: string;
    owner_name: string | null;
    address: string;
    capabilities: Record<string, boolean>;

    city_id: number | null;
    city_name: string | null;

    province_id: number | null;
    province_name: string | null;

    region_id: number | null;
    region_name: string | null;

    zip_code: string | null;
    remarks: string | null;
    contact_numbers: string | null;
    email: string | null;
    schedule: string | null;
    active: boolean | null;

    last_modified_by: string;

    // DateTimeWithTimeZone serialized (commonly ISO string)
    last_modified_on: string;

    // dynamic: "capability_name" -> boolean
}
export type FlattenedClinic = ClinicWithCapabilities & Record<string, any>;


@Injectable({ providedIn: 'root' })
export class DentalClinicService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly baseUrl = environment.apiUrl;

    // Adjust if your backend uses a different path
    private readonly API_PATH = '/api/dental_clinics';

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    private handleError(err: unknown) {
        return throwError(() => err);
    }

    getExtendedClinicCapabilities(): Observable<FlattenedClinic[]> {
        const url = `${this.baseUrl}/api/extended_clinics`;
        return this.http
            .get<ClinicWithCapabilities[]>(url, { headers: this.authHeaders() })
            .pipe(
                tap(rows=>console.log("raw:",rows)),
                map(flattenClinicRows),
                catchError(this.handleError));
    }

    //
    // ---- GET: list (paged)
    //
    // ✅ CHANGED: list returns DentalClinicRow (includes joined names + capability flags)
    getDentalClinics(params: DentalClinicListQuery = {}): Observable<PageResponse<DentalClinicRow>> {
        // ✅ CHANGED: remove trailing slash to avoid route mismatches in some setups
        const url = `${this.baseUrl}${this.API_PATH}/`;

        let httpParams = new HttpParams();

        if (params.page != null) httpParams = httpParams.set('page', String(params.page));
        if (params.page_size != null) httpParams = httpParams.set('page_size', String(params.page_size));

        if (params.city_id != null) httpParams = httpParams.set('city_id', String(params.city_id));
        if (params.active != null) httpParams = httpParams.set('active', String(params.active));
        if (params.name_like != null && params.name_like.trim().length > 0) {
            httpParams = httpParams.set('name_like', params.name_like.trim());
        }

        return this.http
            .get<PageResponse<DentalClinicRow>>(url, { params: httpParams, headers: this.authHeaders() })
            .pipe(catchError(this.handleError));
    }

    //
    // ---- GET: by id
    //
    // ✅ CHANGED: handler returns DentalClinicRowDb (no capability flags)
    getDentalClinicById(id: number): Observable<DentalClinicRowDb> {
        const url = `${this.baseUrl}${this.API_PATH}/${encodeURIComponent(id)}`;
        return this.http
            .get<DentalClinicRowDb>(url, { headers: this.authHeaders() })
            .pipe(catchError(this.handleError));
    }

    //
    // ---- POST: create
    //
    // ✅ CHANGED: handler returns dental_clinic::Model (no joined fields / no flags)
    createDentalClinic(body: CreateDentalClinicBody): Observable<DentalClinicModel> {
        const url = `${this.baseUrl}${this.API_PATH}/`;
        return this.http
            .post<DentalClinicModel>(url, body, { headers: this.authHeaders() })
            .pipe(catchError(this.handleError));
    }

    //
    // ---- PATCH: partial update
    //
    // ✅ CHANGED: handler returns dental_clinic::Model (no joined fields / no flags)
    patchDentalClinic(id: number, body: PatchDentalClinicBody): Observable<DentalClinicModel> {
        const url = `${this.baseUrl}${this.API_PATH}/${encodeURIComponent(id)}`;
        return this.http
            .patch<DentalClinicModel>(url, body, { headers: this.authHeaders() })
            .pipe(catchError(this.handleError));
    }

}

function flattenClinicRows( clinics: ClinicWithCapabilities[]):FlattenedClinic[] {
    return clinics.map(clinic=> ({
        ...clinic,
        ...clinic.capabilities,
    }));
}

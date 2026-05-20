import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TestStackedBar } from './test-stacked-bar';

describe('TestStackedBar', () => {
  let component: TestStackedBar;
  let fixture: ComponentFixture<TestStackedBar>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestStackedBar]
    })
    .compileComponents();

    fixture = TestBed.createComponent(TestStackedBar);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
